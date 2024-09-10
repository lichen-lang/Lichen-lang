use crate::abs::ast::ExprElem;

#[derive(Clone, Debug)]
pub struct ExprBranch {
    pub code_list: ExprElem,
    pub depth: isize,
    pub loopdepth: isize,
}
