use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::comma_parser::CommaParser;
use crate::parser::core_parser::Parser;
use crate::parser::expr_parser::ExprParser;

/// #ListBlockBranch
/// listを格納するためのデータstruct
/// 中では式を解析するパーサを呼び出す必要がある
#[derive(Clone, Debug)]
pub struct ListBlockBranch {
    pub contents: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for ListBlockBranch {
    fn show(&self) {
        println!("{}ListBlock (", " ".repeat(self.depth as usize),);
        for i in &self.contents {
            i.show();
        }
        println!(")");
    }

    fn get_show_as_string(&self) -> String {
        let list_name = format!("{}ListBlock (", " ".repeat(self.depth as usize));
        let mut contents_group = String::new();
        for i in &self.contents {
            contents_group = format!("{}{}", contents_group, i.get_show_as_string());
        }
        let close_paren = format!("{})", " ".repeat(self.depth as usize));
        format!("{}{}{}", list_name, contents_group, close_paren)
    }
}

impl ASTAreaBranch for ListBlockBranch {
    fn new(contents: Option<Vec<ExprElem>>, depth: isize, loopdepth: isize) -> Self {
        Self {
            contents: if let Some(s) = contents { s } else { vec![] },
            depth,
            loopdepth,
        }
    }
}

impl RecursiveAnalysisElements for ListBlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        let mut c_parser =
            CommaParser::create_parser_from_vec(self.contents.clone(), self.depth, self.loopdepth);
        c_parser.code2vec()?;
        c_parser.resolve()?;
        let mut e_parser =
            ExprParser::create_parser_from_vec(self.contents.clone(), self.depth, self.loopdepth);
        e_parser.code2vec()?;
        e_parser.resolve()?;
        self.contents = e_parser.code_list;
        Ok(())
    }
}
