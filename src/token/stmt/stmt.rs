use crate::abs::ast::{ASTBranch, RecursiveAnalysisElements, StmtElem};
use crate::errors::parser_errors::ParserError;

#[derive(Clone, Debug)]
pub struct StmtBranch {
    pub code_list: Vec<StmtElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for StmtBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}

impl ASTBranch for StmtBranch {
    fn show(&self) {
        todo!()
    }

    fn get_show_as_string(&self) -> String {
        todo!()
    }
}
