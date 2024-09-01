use crate::abs::ast::*;

#[derive(Clone, Debug)]
pub enum PrimitiveType {
    I32, // i32
    I64, // i64
    F32, // f32
    F64, // f64
}

impl PrimitiveType {
    fn show(&self) {
        println!("{}", self.get_show_as_string());
    }

    fn get_show_as_string(&self) -> String {
        String::from(match self {
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::F32 => "f32",
            Self::F64 => "f64",
        })
    }
}

#[derive(Clone, Debug)]
pub struct PrimitiveBranch {
    pub primitive_type: PrimitiveType,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for PrimitiveBranch {
    fn show(&self) {
        self.primitive_type.show();
    }

    fn get_show_as_string(&self) -> String {
        self.primitive_type.get_show_as_string()
    }
}
