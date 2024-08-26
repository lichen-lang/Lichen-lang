use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::token::syntax::SyntaxBranch;

/// # SyntaxBoxBranch
/// まとまった文法として解釈される`if elif else` `while else` `for else`などの文法をまとめる
#[derive(Clone, Debug)]
pub struct SyntaxBoxBranch {
    pub name: String,
    pub contents: Vec<SyntaxBranch>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for SyntaxBoxBranch {
    fn show(&self) {
        println!("{}", self.name);
        for i in &self.contents {
            i.show();
        }
    }

    fn get_show_as_string(&self) -> String {
        let mut syntax_string = String::new();
        for i in &self.contents {
            syntax_string = format!("{}{}", syntax_string, i.get_show_as_string());
        }
        format!("{}{}", self.name, syntax_string)
    }
}

impl RecursiveAnalysisElements for SyntaxBoxBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}
