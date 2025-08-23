mod parameters;

use crate::parameters::ParameterUse::{Argument, ReturnElement, ReturnValue};
use anyhow::anyhow;
use convert_case::{Case, Casing};
use piglet_client::{
    client::{Error::ConnectionError, RobotClient},
    dynamic_object::{DynamicObject, Method, Object},
    object_address::ObjectAddress,
    values::PigletCodec,
};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::{fs, fs::File};

struct GeneratedModule {
    name: String,
    addresses: Vec<ObjectAddress>,
    enum_defs: Vec<String>,
    method_defs: Vec<String>,
    struct_defs: Vec<String>,
    unknown: bool,
    version: String,
}

struct MethodGroup {
    methods: Vec<Method>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        anyhow::bail!("Usage: {} <ip> <name>", args[0]);
    }

    println!("Connecting to {}...", args[1]);
    let robot = Arc::new(
        RobotClient::connect(&args[1])
            .await
            .map_err(|e| anyhow!("Error connecting to robot: {}", e))?,
    );

    let mut global_enums = HashMap::new();
    let mut global_imports = Vec::new();
    let mut global_structs = HashMap::new();
    for root in &robot.globals {
        let dynamic = DynamicObject::new(&root, &robot);
        let mut object = dynamic.get_object().await?;
        object.name = object.name.to_case(Case::Pascal);
        let module = object.name.from_case(Case::Pascal).to_case(Case::Snake);

        let interfaces = dynamic.get_interfaces().await?;
        for interface in interfaces {
            let mut enumerations = dynamic.get_enums(interface.id).await?;
            for e in &mut enumerations {
                e.name = format_type_name(&e.name);
                global_enums.insert(global_enums.len() as u8, e.name.clone());
                global_imports.push(format!("{}::{}", module, e.name));
            }

            let mut structs = dynamic.get_structs(interface.id).await?;
            for s in &mut structs {
                s.name = format_type_name(&s.name);
                global_structs.insert(global_structs.len() as u8, s.name.clone());
                global_imports.push(format!("{}::{}", module, s.name));
            }
        }
    }

    let mut modules = HashMap::new();
    for address in &robot.globals {
        build_modules(
            &address,
            &robot,
            &"".to_string(),
            &global_enums,
            &global_structs,
            &mut modules,
        )
        .await?;
    }
    for address in &robot.objects {
        build_modules(
            &address,
            &robot,
            &"".to_string(),
            &global_enums,
            &global_structs,
            &mut modules,
        )
        .await?;
    }
    Arc::try_unwrap(robot)
        .map_err(|_| anyhow!("Error closing robot"))?
        .close()
        .await?;

    let root = format!("piglet_generated/src/{}", args[2]);
    fs::create_dir_all(&root).await?;

    let mut module_files = Vec::new();
    for module in modules.values() {
        let filename = module.name.from_case(Case::Pascal).to_case(Case::Snake);
        module_files.push(filename.clone());

        let mut file = File::create(format!("{}/{}.rs", root, filename)).await?;
        dump_module(&module, &global_imports, &mut file, &filename, &args[2])
            .await
            .map_err(|e| ConnectionError(e))?;
    }

    let mut file = File::create(format!("piglet_generated/src/{}.rs", args[2])).await?;
    file.write_all("#![allow(unused)]\n\n".as_bytes()).await?;
    file.write_all(
        module_files
            .iter()
            .map(|m| format!("pub mod {};\n", m))
            .collect::<Vec<String>>()
            .join("")
            .as_bytes(),
    )
    .await?;

    Ok(())
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct EnumId {
    enum_id: u8,
    interface_id: u8,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct StructId {
    struct_id: u8,
    interface_id: u8,
}

#[derive(Clone, Debug)]
struct Parameter {
    name: String,
    rust_type: String,
    wrap_in_mvec: bool,
}

async fn build_modules(
    address: &ObjectAddress,
    robot: &Arc<RobotClient>,
    prefix: &String,
    global_enums: &HashMap<u8, String>,
    global_structs: &HashMap<u8, String>,
    modules: &mut HashMap<String, GeneratedModule>,
) -> Result<(), anyhow::Error> {
    let dynamic = DynamicObject::new(&address, &robot);
    let mut object = dynamic.get_object().await?;
    println!("{:?}", object);
    object.name = object.name.to_case(Case::Pascal);

    let prefixed = format!("{}{}", prefix, object.name);
    if let Some(existing) = modules.get_mut(&prefixed) {
        existing.addresses.push(address.clone());
    } else {
        modules.insert(
            prefixed.clone(),
            generate_module(&object, &prefixed, &dynamic, global_enums, global_structs).await?,
        );
    }

    for i in 0..(object.subobject_count) {
        let subobject = dynamic.get_subobject_address(i).await?;
        Box::pin(build_modules(
            &subobject,
            &robot,
            &prefixed,
            &global_enums,
            &global_structs,
            modules,
        ))
        .await?;
    }

    Ok(())
}

async fn generate_module(
    object: &Object,
    name: &String,
    dynamic: &DynamicObject,
    global_enums: &HashMap<u8, String>,
    global_structs: &HashMap<u8, String>,
) -> Result<GeneratedModule, anyhow::Error> {
    let address = &object.address;
    let interfaces_result = dynamic.get_interfaces().await;
    if let Err(_) = interfaces_result {
        return Ok(GeneratedModule {
            name: name.clone(),
            addresses: vec![address.clone()],
            enum_defs: vec![],
            method_defs: vec![],
            struct_defs: vec![],
            unknown: true,
            version: object.version.clone(),
        });
    }

    let interfaces = interfaces_result?;
    let mut enums = HashMap::new();
    let mut enum_defs = Vec::new();
    let mut method_defs = Vec::new();
    let mut structs = HashMap::new();
    let mut struct_defs = Vec::new();
    for interface in &interfaces {
        let mut enumerations = dynamic.get_enums(interface.id).await?;
        for i in 0..enumerations.len() {
            let e = &mut enumerations[i];
            e.name = format_type_name(&e.name);
            enums.insert(
                EnumId {
                    interface_id: interface.id,
                    enum_id: i as u8,
                },
                e.name.clone(),
            );

            let mut def = vec![format!(
                r#"
#[derive(Clone, Copy, Debug)]
pub enum {} {{
"#,
                e.name
            )];
            for v in 0..e.labels.len() {
                def.push(format!(
                    "  {} = {},",
                    e.labels[v].from_case(Case::Constant).to_case(Case::Pascal),
                    e.values[v]
                ));
            }

            def.push("}\n".to_string());

            def.push(format!(
                r#"
impl TryFrom<i32> for {} {{
  type Error = piglet_client::client::Error;

  fn try_from(v: i32) -> Result<Self, Self::Error> {{
    match v {{
"#,
                e.name
            ));
            for v in 0..e.labels.len() {
                def.push(format!(
                    "      {} => Ok({}::{}),",
                    e.values[v],
                    e.name,
                    e.labels[v].from_case(Case::Constant).to_case(Case::Pascal)
                ));
            }
            def.push(format!(
                r#"
      _ => Err(ConnectionError(anyhow!("Unknown {} value {{}}", v))),
    }}
  }}
}}

impl PigletCodec for {} {{
  const TYPE_ID: u8 = 32;

  fn serialize(&self, stream: &mut BytesMut) {{
    stream.put_u8(Self::TYPE_ID);
    stream.put_u8(0);
    stream.put_u16_le(4);
    stream.put_i32_le(*self as i32);
  }}

  fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {{
    let type_id = stream.get_u8();
    if Self::TYPE_ID != type_id {{
        return Err(ConnectionError(anyhow!("Expected {{}} but got {{}}", Self::TYPE_ID, type_id)));
    }}
    let _flags = stream.get_u8();
    let length = stream.get_u16_le() as usize;
    let mut bytes = stream.copy_to_bytes(length);
    stream.get_i32_le().try_into()
  }}
}}

impl PigletCodec for MVec<Vec<{}>> {{
  const TYPE_ID: u8 = 35;
  fn serialize(&self, bytes: &mut BytesMut) {{
    bytes.put_u8(Self::TYPE_ID);
    bytes.put_u8(0);
    bytes.put_u16_le(4 * self.0.len() as u16);
    for v in &self.0 {{
      bytes.put_i32_le(*v as i32);
    }}
  }}

  fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {{
    let type_id = stream.get_u8();
    if Self::TYPE_ID != type_id {{
        return Err(ConnectionError(anyhow!("Expected {{}} but got {{}}", Self::TYPE_ID, type_id)));
    }}
    let _flags = stream.get_u8();
    let length = stream.get_u16_le() as usize;
    let mut arr = Vec::new();
    for i in 0..(length / 4) {{
      arr.push(stream.get_i32_le().try_into()?);
    }}
    Ok(MVec(arr))
  }}
}}

"#,
                e.name, e.name, e.name
            ));

            enum_defs.push(def.join("\n"));
        }
    }

    for interface in &interfaces {
        let structures = dynamic.get_structs(interface.id).await?;
        for i in 0..structures.len() {
            let s = &structures[i];
            structs.insert(
                StructId {
                    interface_id: interface.id,
                    struct_id: i as u8,
                },
                s.name.clone(),
            );

            let mut def = vec![format!(
                r#"
#[derive(Clone, Debug)]
pub struct {} {{
"#,
                s.name
            )];
            let mut deserializes = vec![];
            let mut serializes = vec![];
            let mut type_offset = 0;
            for m in 0..s.element_labels.len() {
                let rust_type = match s.element_types[type_offset] {
                    i8::TYPE_ID => "i8",
                    i16::TYPE_ID => "i16",
                    i32::TYPE_ID => "i32",
                    u8::TYPE_ID => "u8",
                    u16::TYPE_ID => "u16",
                    u32::TYPE_ID => "u32",
                    String::TYPE_ID => "String",
                    Vec::<u8>::TYPE_ID => "Vec::<u8>",
                    bool::TYPE_ID => "bool",
                    Vec::<i16>::TYPE_ID => "Vec::<i16>",
                    Vec::<u16>::TYPE_ID => "Vec::<u16>",
                    Vec::<i32>::TYPE_ID => "Vec::<i32>",
                    Vec::<u32>::TYPE_ID => "Vec::<u32>",
                    Vec::<bool>::TYPE_ID => "Vec::<bool>",
                    30 => {
                        let source_id = s.element_types[type_offset + 1];
                        let id = s.element_types[type_offset + 2];
                        let e = match source_id {
                            1 => &global_structs[&(id - 1)],
                            2 => {
                                &structs[&StructId {
                                    interface_id: interface.id,
                                    struct_id: id - 1,
                                }]
                            }
                            3 => "NetworkResult",
                            _ => anyhow::bail!("Unknown source {}", source_id),
                        };
                        type_offset += 2;
                        e
                    }
                    31 => {
                        let source_id = s.element_types[type_offset + 1];
                        let id = s.element_types[type_offset + 2];
                        let e = match source_id {
                            1 => &global_structs[&(id - 1)],
                            2 => {
                                &structs[&StructId {
                                    interface_id: interface.id,
                                    struct_id: id - 1,
                                }]
                            }
                            3 => "NetworkResult",
                            _ => anyhow::bail!("Unknown source {}", source_id),
                        };
                        type_offset += 2;
                        &format!("Vec::<{}>", e)
                    }
                    32 => {
                        let source_id = s.element_types[type_offset + 1];
                        let id = s.element_types[type_offset + 2];
                        let e = match source_id {
                            1 => &global_enums[&(id - 1)],
                            2 => {
                                &enums[&EnumId {
                                    interface_id: interface.id,
                                    enum_id: id - 1,
                                }]
                            }
                            _ => anyhow::bail!("Unknown source {}", source_id),
                        };
                        type_offset += 2;
                        e
                    }
                    33 => "piglet_client::values::ErrorCode",
                    35 => {
                        let source_id = s.element_types[type_offset + 1];
                        let id = s.element_types[type_offset + 2];
                        let e = match source_id {
                            1 => &global_enums[&(id - 1)],
                            2 => {
                                &enums[&EnumId {
                                    interface_id: interface.id,
                                    enum_id: id - 1,
                                }]
                            }
                            _ => anyhow::bail!("Unknown source {}", source_id),
                        };
                        type_offset += 2;
                        &format!("Vec::<{}>", e)
                    }
                    Vec::<String>::TYPE_ID => "Vec::<String>",
                    _ => anyhow::bail!("Unknown value type {}", s.element_types[type_offset]),
                };
                let name = format_member_name(&s.element_labels[m]);
                def.push(format!("  pub {}: {},", name, rust_type));

                let (deserialize, serialize) = match s.element_types[type_offset] {
                    31 | 35 => (
                        format!(
                            "{}: MVec::<{}>::deserialize(&mut bytes)?.0",
                            name, rust_type
                        ),
                        format!("MVec(s.{}).serialize(&mut buffer)", name),
                    ),
                    _ => (
                        format!("{}: {}::deserialize(&mut bytes)?", name, rust_type),
                        format!("s.{}.serialize(&mut buffer)", name),
                    ),
                };
                deserializes.push(deserialize);
                serializes.push(serialize);
                type_offset += 1;
            }
            def.push(format!(
                r#"
}}

impl PigletCodec for {} {{
  const TYPE_ID: u8 = 30;

  fn serialize(&self, stream: &mut BytesMut) {{
    stream.put_u8(Self::TYPE_ID);
    stream.put_u8(0);
    let mut buffer = BytesMut::new();
    let s = self;
"#,
                s.name
            ));

            for serialize in &serializes {
                def.push(format!("    {};", serialize));
            }

            def.push(format!(
                r#"
    stream.put_u16_le(buffer.len() as u16);
    stream.put(buffer);
  }}

  fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {{
    let type_id = stream.get_u8();
    if Self::TYPE_ID != type_id {{
        return Err(ConnectionError(anyhow!("Expected {{}} but got {{}}", Self::TYPE_ID, type_id)));
    }}
    let _flags = stream.get_u8();
    let length = stream.get_u16_le() as usize;
    let mut bytes = stream.copy_to_bytes(length);
    Ok(Self {{
"#
            ));

            for deserialize in &deserializes {
                def.push(format!("      {},", deserialize));
            }

            def.push(format!(
                r#"
    }})
  }}
}}

impl PigletCodec for MVec<Vec<{}>> {{
  const TYPE_ID: u8 = 31;
  fn serialize(&self, stream: &mut BytesMut) {{
    stream.put_u8(Self::TYPE_ID);
    stream.put_u8(0);

    let mut outer = BytesMut::new();
    for s in &self.0 {{
        let mut buffer = BytesMut::new();
"#,
                s.name
            ));

            for serialize in &serializes {
                def.push(format!("    {};", serialize));
            }

            def.push(format!(
                r#"
        outer.put_u16_le(buffer.len() as u16);
        outer.put(buffer);
    }}

    stream.put_u16_le(outer.len() as u16);
    stream.put(outer);
  }}

  fn deserialize(stream: &mut Bytes) -> Result<Self, Error> {{
    let type_id = stream.get_u8();
    if Self::TYPE_ID != type_id {{
        return Err(ConnectionError(anyhow!("Expected {{}} but got {{}}", Self::TYPE_ID, type_id)));
    }}
    let _flags = stream.get_u8();
    let mut length = stream.get_u16_le() as usize;
    let mut outer = stream.copy_to_bytes(length);
    let mut arr = Vec::new();
    while length > 0 {{
        let mut length = outer.get_u16_le() as usize;
        let mut bytes = outer.copy_to_bytes(length);
        arr.push({} {{
"#,
                s.name
            ));

            for deserialize in &deserializes {
                def.push(format!("          {},", deserialize));
            }

            def.push(format!(
                r#"
        }});
    }}
    Ok(MVec(arr))
  }}
}}
"#
            ));
            struct_defs.push(def.join("\n"));
        }
    }

    let mut methods = HashMap::<String, MethodGroup>::new();
    for i in 0..(object.method_count) {
        let method = dynamic.get_method(i).await?;
        println!("{:?}", method);
        if let Some(existing) = methods.get_mut(&method.name) {
            existing.methods.push(method);
        } else {
            methods.insert(
                method.name.clone(),
                MethodGroup {
                    methods: vec![method],
                },
            );
        }
    }

    let mut flatten = Vec::new();
    for group in methods.values() {
        if group.methods.len() == 1 {
            flatten.push(group.methods[0].clone());
        } else {
            for i in 0..group.methods.len() {
                let mut method = group.methods[0].clone();
                method.name = format!("{}_{}", method.name, i + 1);
                flatten.push(method);
            }
        }
    }

    flatten.sort_by(|a, b| {
        // We want to ensure that the generic methods on interface 0 get put at the bottom
        let a_key: u32 = match a.interface_id {
            0 => 1024 * 1024 + a.method_id as u32,
            i => (i as u32) * 1024 + a.method_id as u32,
        };
        let b_key: u32 = match b.interface_id {
            0 => 1024 * 1024 + b.method_id as u32,
            i => (i as u32) * 1024 + b.method_id as u32,
        };
        return a_key.cmp(&b_key);
    });

    for method in flatten {
        let mut arguments = Vec::new();
        let mut return_elements = Vec::new();
        let mut return_values = Vec::new();
        let mut parameter_i = 0;
        for label in &method.parameter_labels {
            let name = format_parameter_name(&label);
            let raw_type = method.parameter_types[parameter_i];
            let parsed_type = match raw_type {
                1 => Argument {
                    rust_type: "i8".to_string(),
                },
                2 => Argument {
                    rust_type: "u8".to_string(),
                },
                3 => Argument {
                    rust_type: "i16".to_string(),
                },
                4 => Argument {
                    rust_type: "u16".to_string(),
                },
                5 => Argument {
                    rust_type: "i32".to_string(),
                },
                6 => Argument {
                    rust_type: "u32".to_string(),
                },
                7 => Argument {
                    rust_type: "String".to_string(),
                },
                8 => Argument {
                    rust_type: "Vec::<u8>".to_string(),
                },
                18 => ReturnElement {
                    rust_type: "u8".to_string(),
                },
                19 => ReturnElement {
                    rust_type: "i16".to_string(),
                },
                20 => ReturnElement {
                    rust_type: "u16".to_string(),
                },
                21 => ReturnElement {
                    rust_type: "i32".to_string(),
                },
                22 => ReturnElement {
                    rust_type: "u32".to_string(),
                },
                23 => ReturnElement {
                    rust_type: "String".to_string(),
                },
                24 => ReturnElement {
                    rust_type: "Vec::<u8>".to_string(),
                },
                25 => ReturnValue {
                    rust_type: "i8".to_string(),
                },
                26 => ReturnValue {
                    rust_type: "u8".to_string(),
                },
                27 => ReturnValue {
                    rust_type: "i16".to_string(),
                },
                28 => ReturnValue {
                    rust_type: "u16".to_string(),
                },
                29 => ReturnValue {
                    rust_type: "i32".to_string(),
                },
                30 => ReturnValue {
                    rust_type: "u32".to_string(),
                },
                31 => ReturnValue {
                    rust_type: "String".to_string(),
                },
                32 => ReturnValue {
                    rust_type: "Vec::<u8>".to_string(),
                },
                33 => Argument {
                    rust_type: "bool".to_string(),
                },
                35 => ReturnElement {
                    rust_type: "bool".to_string(),
                },
                36 => ReturnValue {
                    rust_type: "bool".to_string(),
                },
                41 => Argument {
                    rust_type: "Vec::<i16>".to_string(),
                },
                43 => ReturnElement {
                    rust_type: "Vec::<i16>".to_string(),
                },
                44 => ReturnValue {
                    rust_type: "Vec::<i16>".to_string(),
                },
                45 => Argument {
                    rust_type: "Vec::<u16>".to_string(),
                },
                47 => ReturnElement {
                    rust_type: "Vec::<u16>".to_string(),
                },
                48 => ReturnValue {
                    rust_type: "Vec::<u16>".to_string(),
                },
                49 => Argument {
                    rust_type: "Vec::<i32>".to_string(),
                },
                51 => ReturnElement {
                    rust_type: "Vec::<i32>".to_string(),
                },
                52 => ReturnValue {
                    rust_type: "Vec::<i32>".to_string(),
                },
                53 => Argument {
                    rust_type: "Vec::<u32>".to_string(),
                },
                55 => ReturnElement {
                    rust_type: "Vec::<u32>".to_string(),
                },
                56 => ReturnValue {
                    rust_type: "Vec::<u32>".to_string(),
                },
                60 => {
                    let source_id = method.parameter_types[parameter_i + 1];
                    let id = method.parameter_types[parameter_i + 2];
                    let e = match source_id {
                        1 => &global_structs[&(id - 1)],
                        2 => {
                            &structs[&StructId {
                                interface_id: method.interface_id,
                                struct_id: id - 1,
                            }]
                        }
                        _ => anyhow::bail!("Unknown source {}", source_id),
                    };
                    parameter_i += 2;
                    ReturnValue {
                        rust_type: e.clone(),
                    }
                }
                61 => {
                    let source_id = method.parameter_types[parameter_i + 1];
                    let id = method.parameter_types[parameter_i + 2];
                    let e = match source_id {
                        1 => &global_structs[&(id - 1)],
                        2 => {
                            &structs[&StructId {
                                interface_id: method.interface_id,
                                struct_id: id - 1,
                            }]
                        }
                        _ => anyhow::bail!("Unknown source {}", source_id),
                    };
                    parameter_i += 2;
                    Argument {
                        rust_type: format!("Vec::<{}>", e),
                    }
                }
                64 => {
                    let source_id = method.parameter_types[parameter_i + 1];
                    let id = method.parameter_types[parameter_i + 2];
                    let e = match source_id {
                        1 => &global_structs[&(id - 1)],
                        2 => {
                            &structs[&StructId {
                                interface_id: method.interface_id,
                                struct_id: id - 1,
                            }]
                        }
                        _ => anyhow::bail!("Unknown source {}", source_id),
                    };
                    parameter_i += 2;
                    ReturnValue {
                        rust_type: format!("Vec::<{}>", e),
                    }
                }
                66 => Argument {
                    rust_type: "Vec::<bool>".to_string(),
                },
                68 => ReturnElement {
                    rust_type: "Vec::<bool>".to_string(),
                },
                69 => ReturnValue {
                    rust_type: "Vec::<bool>".to_string(),
                },
                76 => ReturnElement {
                    rust_type: "Vec::<String>".to_string(),
                },
                78 => {
                    let source_id = method.parameter_types[parameter_i + 1];
                    let id = method.parameter_types[parameter_i + 2];
                    let e = match source_id {
                        1 => &global_enums[&(id - 1)],
                        2 => {
                            &enums[&EnumId {
                                interface_id: method.interface_id,
                                enum_id: id - 1,
                            }]
                        }
                        _ => anyhow::bail!("Unknown source {}", source_id),
                    };
                    parameter_i += 2;
                    Argument {
                        rust_type: e.clone(),
                    }
                }
                81 => {
                    let source_id = method.parameter_types[parameter_i + 1];
                    let id = method.parameter_types[parameter_i + 2];
                    let e = match source_id {
                        1 => &global_enums[&(id - 1)],
                        2 => {
                            &enums[&EnumId {
                                interface_id: method.interface_id,
                                enum_id: id - 1,
                            }]
                        }
                        _ => anyhow::bail!("Unknown source {}", source_id),
                    };
                    parameter_i += 2;
                    ReturnValue {
                        rust_type: e.clone(),
                    }
                }
                82 => {
                    let source_id = method.parameter_types[parameter_i + 1];
                    let id = method.parameter_types[parameter_i + 2];
                    let e = match source_id {
                        1 => &global_enums[&(id - 1)],
                        2 => {
                            &enums[&EnumId {
                                interface_id: method.interface_id,
                                enum_id: id - 1,
                            }]
                        }
                        _ => anyhow::bail!("Unknown source {}", source_id),
                    };
                    parameter_i += 2;
                    Argument {
                        rust_type: format!("Vec::<{}>", e),
                    }
                }
                85 => {
                    let source_id = method.parameter_types[parameter_i + 1];
                    let id = method.parameter_types[parameter_i + 2];
                    let e = match source_id {
                        1 => &global_enums[&(id - 1)],
                        2 => {
                            &enums[&EnumId {
                                interface_id: method.interface_id,
                                enum_id: id - 1,
                            }]
                        }
                        _ => anyhow::bail!("Unknown source {}", source_id),
                    };
                    parameter_i += 2;
                    ReturnValue {
                        rust_type: format!("Vec::<{}>", e),
                    }
                }
                102 => Argument {
                    rust_type: "f32".to_string(),
                },
                104 => ReturnElement {
                    rust_type: "f32".to_string(),
                },
                105 => ReturnValue {
                    rust_type: "f32".to_string(),
                },
                _ => anyhow::bail!("Unknown type {}", raw_type),
            };
            let wrap_in_mvec = match raw_type {
                61 | 64 | 82 | 85 => true,
                _ => false,
            };
            match parsed_type {
                Argument { rust_type } => arguments.push(Parameter {
                    name,
                    rust_type,
                    wrap_in_mvec,
                }),
                ReturnElement { rust_type } => return_elements.push(Parameter {
                    name,
                    rust_type,
                    wrap_in_mvec,
                }),
                ReturnValue { rust_type } => return_values.push(Parameter {
                    name,
                    rust_type,
                    wrap_in_mvec,
                }),
            }

            parameter_i += 1;
        }

        if return_elements.len() > 0 && return_values.len() > 0 {
            return_elements.extend(return_values.clone());
            return_values.clear();
        } else if return_values.len() > 1 {
            anyhow::bail!("Cannot have more than one return value");
        }

        let mut contents = vec![format!(
            r#"
{}  pub async fn {}(
    &self,
  "#,
            // TODO(april): make an arg to control whether this is on?
            // format!("{:#?}", method)
            //     .lines()
            //     .fold(String::new(), |mut r, l| {
            //         r.push_str("  // ");
            //         r.push_str(l);
            //         r.push('\n');
            //         r
            //     }),
            "",
            method.name.from_case(Case::Pascal).to_case(Case::Snake)
        )];
        for argument in &arguments {
            contents.push(format!("    {}: {},", argument.name, argument.rust_type));
        }

        if return_elements.len() > 0 {
            let mut struct_def = vec![format!(
                r#"
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct {}Reply {{
"#,
                method.name
            )];
            for e in &return_elements {
                struct_def.push(format!("  {}: {},", e.name, e.rust_type));
            }
            struct_def.push("}".to_string());
            struct_defs.push(struct_def.join("\n"));
        }

        let return_type = if return_elements.len() > 0 {
            format!("{}Reply", method.name)
        } else if return_values.len() == 1 {
            let value = &return_values[0];
            format!("/* {}= */ {}", value.name, value.rust_type)
        } else {
            "()".to_string()
        };

        contents.push(format!("  ) -> Result<{}, Error> {{", return_type));
        contents.push("    let mut args = BytesMut::new();".to_string());
        for argument in &arguments {
            let source = if argument.wrap_in_mvec {
                format!("MVec({})", argument.name)
            } else {
                argument.name.clone()
            };
            contents.push(format!("    {}.serialize(&mut args);", source));
        }
        contents.push(
            format!(
                "    let (count, mut stream) = self.robot.act(&self.address, {}, {}, {}, args.freeze()).await?;",
                method.interface_id,
                method.call_type,
                method.method_id,
            ));
        let expected_length = return_elements.len() + return_values.len();
        contents.push(format!("    if count != {} {{", expected_length));
        contents.push(format!(
            "      return Err(ConnectionError(anyhow!(\"Expected {} values, not {{}}\", count)));",
            expected_length
        ));
        contents.push("    }".to_string());

        for e in return_elements.iter().chain(&return_values) {
            let wrapped = if e.wrap_in_mvec {
                format!("MVec::<{}>", e.rust_type)
            } else {
                e.rust_type.clone()
            };
            let suffix = if e.wrap_in_mvec { ".0" } else { "" };
            contents.push(format!(
                "    let {} = {}::deserialize(&mut stream)?{};",
                e.name.from_case(Case::Camel).to_case(Case::Snake),
                wrapped,
                suffix
            ));
        }

        let ret = if return_elements.len() > 0 {
            format!(
                "{}Reply {{ {} }}",
                method.name,
                return_elements
                    .into_iter()
                    .map(|e| e.name.from_case(Case::Camel).to_case(Case::Snake))
                    .collect::<Vec::<_>>()
                    .join(", ")
            )
        } else if return_values.len() == 1 {
            let value = &return_values[0];
            value.name.from_case(Case::Camel).to_case(Case::Snake)
        } else {
            "()".to_string()
        };
        contents.push(format!("    Ok({})", ret));

        contents.push("  }".to_string());

        method_defs.push(contents.join("\n"));
    }

    Ok(GeneratedModule {
        name: name.clone(),
        addresses: vec![address.clone()],
        enum_defs,
        method_defs,
        struct_defs,
        unknown: false,
        version: object.version.clone(),
    })
}

async fn dump_module(
    module: &GeneratedModule,
    global_imports: &Vec<String>,
    file: &mut File,
    filename: &String,
    parent: &String,
) -> Result<(), anyhow::Error> {
    let name = &module.name;
    let addresses = &module.addresses;
    let silence = format!("{}::", filename);
    for i in global_imports {
        if !i.starts_with(&silence) {
            file.write_all(format!("use crate::{}::{};\n", parent, i).as_bytes())
                .await?;
        }
    }

    file.write_all(
        format!(
            r#"
use anyhow::anyhow;
use bytes::{{Buf, BufMut, Bytes, BytesMut}};
use crate::traits::MVec;
use piglet_client::{{
  client::{{Error, Error::ConnectionError, RobotClient}},
  object_address::ObjectAddress,
  values::{{PigletCodec, NetworkResult}},
}};
use std::sync::Arc;

pub struct {} {{
  address: ObjectAddress,
  robot: Arc<RobotClient>,
}}

impl {} {{

  // version {}
"#,
            name, name, module.version
        )
        .as_bytes(),
    )
    .await?;

    if addresses.len() == 1 {
        let address = &addresses[0];
        file.write_all(
            format!(
                r#"
  pub fn new(robot: &Arc<RobotClient>) -> Self {{
    Self {{
      address: {:?},
      robot: robot.clone(),
    }}
  }}
"#,
                address
            )
            .as_bytes(),
        )
        .await?;
    } else {
        for i in 0..addresses.len() {
            let address = &addresses[i];
            file.write_all(
                format!(
                    r#"
  pub fn new_{}(robot: &Arc<RobotClient>) -> Self {{
    Self {{
      address: {:?},
      robot: robot.clone(),
    }}
  }}
"#,
                    i + 1,
                    address
                )
                .as_bytes(),
            )
            .await?;
        }
    }

    file.write_all(
        format!(
            r#"
{}
}}

{}

{}

"#,
            module.method_defs.join("\n\n"),
            module.enum_defs.join("\n\n"),
            module.struct_defs.join("\n\n")
        )
        .as_bytes(),
    )
    .await?;

    if module.unknown {
        file.write_all("// module was not present on the dumping machine".as_bytes())
            .await?;
    }

    Ok(())
}

fn format_type_name(name: &String) -> String {
    // There's a type named "eMOTIONPROFILE" that we'd prefer as EMotionprofile
    name.from_case(Case::Pascal).to_case(Case::Pascal)
}

fn format_member_name(name: &String) -> String {
    if name == "Type" {
        "type_".to_string()
    } else {
        name.from_case(Case::Camel).to_case(Case::Snake)
    }
}

fn format_parameter_name(name: &String) -> String {
    if name == "type" {
        "type_".to_string()
    } else {
        name.from_case(Case::Camel).to_case(Case::Snake)
    }
}
