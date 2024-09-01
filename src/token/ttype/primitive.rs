#[derive(Clone, Debug)]
pub enum PrimitiveType {
    I32, // i32
    I64, // i64
    F32, // f32
    F64, // f64
}

#[derive(Clone, Debug)]
pub struct PrimitiveBranch {
    pub primitive_type: PrimitiveType,
    pub depth: isize,
    pub loopdepth: isize,
}
