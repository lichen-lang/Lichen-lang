use crate::abs::ast::ASTBranch;
use crate::abs::ast::ExprElem;
use crate::abs::ast::RecursiveAnalysisElements;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;
use crate::parser::expr_parser::ExprParser;

#[derive(Clone, Debug)]
pub struct ItemBranch {
    pub contents: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for ItemBranch {
    fn show(&self) {
        for i in &self.contents {
            i.show();
        }
    }

    fn get_show_as_string(&self) -> String {
        let mut rstr = String::new();
        for i in &self.contents {
            rstr = format!("{}{}", rstr, i.get_show_as_string());
        }
        rstr
    }
}

impl RecursiveAnalysisElements for ItemBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        // 式パーサーによって解析
        let mut parser =
            ExprParser::create_parser_from_vec(self.contents.clone(), self.depth, self.loopdepth);
        match parser.code2vec() {
            Ok(_) => {
                let mut rlist = parser.code_list;
                for i in &mut rlist {
                    i.resolve_self()?
                }
                self.contents = rlist;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
