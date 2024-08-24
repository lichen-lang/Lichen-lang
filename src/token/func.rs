use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::comma_parser::CommaParser;
use crate::parser::core_parser::Parser;
use crate::parser::expr_parser::ExprParser;

use super::paren_block::ParenBlockBranch;

/// # FuncBranch
/// 関数呼び出しのトークン
#[derive(Clone, Debug)]
pub struct FuncBranch {
    pub name: Box<ExprElem>,
    pub contents: ParenBlockBranch,
    pub out_code_list: Vec<Vec<ExprElem>>, // args
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for FuncBranch {
    fn show(&self) {
        println!("{}func name", " ".repeat(self.depth as usize * 4));
        self.name.show();
        println!("{}(", " ".repeat(self.depth as usize * 4));
        for (i, j) in self.out_code_list.iter().enumerate() {
            println!("{}arg{}", " ".repeat(self.depth as usize * 4), i);
            for k in j {
                k.show();
            }
        }
        println!("{})", " ".repeat(self.depth as usize * 4));
    }

    fn get_show_as_string(&self) -> String {
        let function_name_section = format!(
            "{}func name\n{}",
            " ".repeat(self.depth as usize * 4),
            self.name.get_show_as_string()
        );
        let paren_open = format!("{}(\n", " ".repeat(self.depth as usize * 4));
        let mut args_group = String::new();
        for (i, j) in self.out_code_list.iter().enumerate() {
            args_group = format!(
                "{}{}arg{}\n",
                args_group,
                " ".repeat(self.depth as usize * 4),
                i
            );
            for k in j {
                args_group = format!("{}{}\n", args_group, k.get_show_as_string());
            }
        }
        let paren_close = format!("{})", " ".repeat(self.depth as usize * 4));
        format!(
            "{}{}{}{}",
            function_name_section, paren_open, args_group, paren_close
        )
    }
}

impl RecursiveAnalysisElements for FuncBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        self.name.resolve_self()?;
        // 呼び出し元の自己解決
        // 引数の解決
        if let ParenBlockBranch {
            contents: Some(v),
            depth,
            loopdepth,
        } = self.contents.clone()
        {
            let mut comma_parser = CommaParser::create_parser_from_vec(v, depth, loopdepth);
            if let ExprElem::OpeElem(_) = &*self.name {
                // もし、関数の名前が演算子だった場合、 `self.out_code_list`に対しては処理をしない
                // pass
            } else {
                comma_parser.resolve()?;
                self.out_code_list = comma_parser.out_code_list;
            }
        }
        for i in &mut self.out_code_list {
            let mut parser =
                ExprParser::create_parser_from_vec(i.to_vec(), self.depth + 1, self.loopdepth);
            parser.code2vec()?;
            for inner in &mut parser.code_list {
                inner.resolve_self()?;
            }
            *i = parser.code_list;
        }
        Ok(())
    }
}
