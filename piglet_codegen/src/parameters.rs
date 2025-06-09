#[derive(Clone, Debug)]
pub enum ParameterUse {
    Argument { rust_type: String },
    ReturnElement { rust_type: String },
    ReturnValue { rust_type: String },
}
