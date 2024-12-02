use crate::abs::ast::*;
use crate::abs::gen::Wasm_gen;
use crate::errors::generate_errors::GenerateError;
use crate::errors::parser_errors::ParserError;
use crate::gen::wasm::MEMORY_SPACE_NAME;
use crate::parser::expr_parser::ExprParser;

/// 引数などの式を格納します
#[derive(Clone, Debug)]
pub struct ItemBranch {
    pub contents: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ItemBranch {
    /// 何も要素を持たない引数だった場合trueを返却する
    pub fn has_no_elem(&self) -> bool {
        self.contents.is_empty()
    }
}

impl ASTBranch for ItemBranch {
    fn show(&self) {
        println!("{}Item(", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            i.show();
        }
        println!("{})", " ".repeat(self.depth as usize * 4));
    }

    fn get_show_as_string(&self) -> String {
        let mut rstr = format!("{}Item(\n", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            rstr = format!("{}{}", rstr, i.get_show_as_string());
        }
        rstr = format!("{}\n{})", rstr, " ".repeat(self.depth as usize * 4));
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
