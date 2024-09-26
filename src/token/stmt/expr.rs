use crate::abs::ast::{ ASTBranch, ExprElem, RecursiveAnalysisElements, Token};
use crate::errors::parser_errors::ParserError;
use crate::parser::expr_parser::ExprParser;

#[derive(Clone, Debug)]
pub struct ExprBranch {
    pub code_list: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for ExprBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        let mut parser =
            ExprParser::create_parser_from_vec(self.code_list.clone(), self.depth, self.loopdepth);
        match parser.code2vec() {
            Ok(_) => {
                let mut rlist = parser.code_list;
                for i in &mut rlist {
                    i.resolve_self()?;
                }
                self.code_list = rlist;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl ASTBranch for ExprBranch {
    fn show(&self) {
        //pass
        for i in &self.code_list {
            i.show();
        }
    }

    fn get_show_as_string(&self) -> String {
        //pass
        let mut rlist: String = String::new();
        for i in &self.code_list {
            rlist = format!("{}{}", rlist, i.get_show_as_string());
        }
        rlist
    }
}
