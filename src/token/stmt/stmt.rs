use crate::abs::ast::StmtElem;

#[derive(Clone, Debug)]
pub struct StmtBranch {
    pub code_list: Vec<StmtElem>,
    pub depth: isize,
    pub loopdepth: isize,
}
