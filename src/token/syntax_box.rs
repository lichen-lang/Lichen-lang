use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::token::syntax::SyntaxBranch;

/// # SyntaxBoxBranch
/// まとまった文法として解釈される`if elif else` `while else` `for else`などの文法をまとめる
#[cfg(debug_assertions)]
#[derive(Clone, Debug)]
pub struct SyntaxBoxBranch {
    pub name: String,
    pub contents: Vec<SyntaxBranch>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for SyntaxBoxBranch {
    fn show(&self) {
        todo!()
    }
    fn get_show_as_string(&self) -> String {
        todo!()
    }
}

impl RecursiveAnalysisElements for SyntaxBoxBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}
