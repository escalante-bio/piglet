// Define a type for borrowing slices
pub(crate) struct MSlice<'a, E>(pub &'a [E]);

// Define a Vec type we can implement deserialize for
pub(crate) struct MVec<E>(pub Vec<E>);
