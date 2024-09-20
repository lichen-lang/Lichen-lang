use crate::abs::ast::ExprElem;

#[derive(Clone, Debug)]
pub struct LetBranch {
    pub name: ExprElem,
    pub code_list: Option<ExprElem>,
    pub public: bool,
    pub export: bool,
    pub depth: isize,
    pub loopdepth: isize,
}
