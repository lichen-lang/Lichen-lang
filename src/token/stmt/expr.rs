use crate::abs::ast::ExprElem;

#[derive(Clone, Debug)]
pub struct ExprBranch {
    pub code_list: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}
