use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::stmt_parser::*;


#[derive(Clone, Debug)]
pub struct DecFuncBranch{
    pub contents: Vec<StmtElem>,
    pub depth:isize,
    pub loopdepth:isize
}

impl RecursiveAnalysisElements for DecFuncBranch{
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}

