use crate::abs::ast::*;

use crate::errors::parser_errors::ParserError;
use crate::parser::comma_parser::CommaParser;
use crate::parser::core_parser::Parser;

use crate::token::list_block::ListBlockBranch;

#[derive(Clone, Debug)]
pub struct ListBranch {
    pub name: Box<ExprElem>,
    pub contents: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for ListBranch {
    fn show(&self) {
        println!("{}List name", " ".repeat(self.depth as usize * 4));
        self.name.show();
        println!("{}(", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            i.show();
        }
        println!("{})", " ".repeat(self.depth as usize * 4));
    }

    fn get_show_as_string(&self) -> String {
        let function_name_section = format!(
            "{}List name\n{}",
            " ".repeat(self.depth as usize * 4),
            self.name.get_show_as_string()
        );
        let paren_open = format!("{}(\n", " ".repeat(self.depth as usize * 4));
        let mut list_items = String::new();
        for i in &self.contents {
            list_items = format!("{}{}", list_items, i.get_show_as_string());
        }
        let paren_close = format!("{})\n", " ".repeat(self.depth as usize * 4));

        format!(
            "{}{}{}{}",
            paren_open, function_name_section, list_items, paren_close,
        )
    }
}

impl RecursiveAnalysisElements for ListBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        self.name.resolve_self()?;

        if self.contents.len() != 1 {
            return Err(ParserError::DevError);
        }
        let first_elem = &self.contents[0];

        if let ExprElem::ListBlockElem(ListBlockBranch {
            contents: v,
            depth,
            loopdepth,
        }) = first_elem
        {
            let mut c_parser = CommaParser::create_parser_from_vec(v.to_vec(), *depth, *loopdepth);
            c_parser.resolve()?;
            self.contents = c_parser.code_list;
        } else {
            return Err(ParserError::DevError);
        }
        for inner in &mut self.contents {
            inner.resolve_self()?;
        }
        Ok(())
    }
}
