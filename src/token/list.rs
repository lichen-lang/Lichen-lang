use crate::abs::ast::*;

use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;
use crate::parser::expr_parser::ExprParser;

use super::list_block::ListBlockBranch;
use super::paren_block::ParenBlockBranch;

#[derive(Clone, Debug)]
pub struct ListBranch {
    pub name: Box<BaseElem>,
    pub contents: ListBlockBranch,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for ListBranch {
    fn show(&self) {
        println!("{}List name", " ".repeat(self.depth as usize * 4));
        self.name.show();
        println!("{}(", " ".repeat(self.depth as usize * 4));
        self.contents.show();
        println!("{})", " ".repeat(self.depth as usize * 4));
    }

    fn get_show_as_string(&self) -> String {
        let function_name_section = format!(
            "{}List name\n{}",
            " ".repeat(self.depth as usize * 4),
            self.name.get_show_as_string()
        );
        let paren_open = format!("{}(\n", " ".repeat(self.depth as usize * 4));
        let paren_close = format!("{})\n", " ".repeat(self.depth as usize * 4));

        format!(
            "{}{}{}{}",
            paren_open,
            function_name_section,
            self.contents.get_show_as_string(),
            paren_close,
        )
    }
}

impl RecursiveAnalysisElements for ListBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        self.name.resolve_self()?;
        // self.contents.resolve_self()?;
        self.contents.resolve_self()?;
        Ok(())
    }
}
