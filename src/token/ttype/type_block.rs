use crate::abs::ast::*;

#[derive(Clone, Debug)]
pub struct TypeBlockBranch<T> {
    pub code_list: Vec<T>,
    pub depth: isize,
    // loopdepth: isize,
}

impl<T> TypeBlockBranch<T> {}
