use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;
use crate::parser::expr_parser::ExprParser;
use crate::token::paren_block::ParenBlockBranch;

/// # FuncBranch
/// 関数呼び出しのトークン
/// ```
/// f(args)
/// ```
#[derive(Clone)]
pub struct FuncBranch {
    pub name: Box<BaseElem>,
    pub contents: Vec<Vec<BaseElem>>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for FuncBranch {
    fn show(&self) {
        println!("{}func name", " ".repeat(self.depth as usize * 4));
        self.name.show();
        println!("{}(", " ".repeat(self.depth as usize * 4));
        for (i, j) in self.contents.iter().enumerate() {
            print!("{}arg{}\n", " ".repeat(self.depth as usize * 4), i);
            for k in j {
                k.show();
            }
        }
        println!("{})", " ".repeat(self.depth as usize * 4));
    }
}

impl RecursiveAnalysisElements for FuncBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        if let Err(e) = self.name.resolve_self() {
            return Err(e);
        } // 呼び出し元の自己解決
          // 引数の解決
        for i in &mut self.contents {
            let mut parser =
                ExprParser::create_parser_from_vec(i.to_vec(), self.depth, self.loopdepth);
            if let Err(e) = parser.code2vec() {
                return Err(e);
            }
            for inner in &mut parser.code_list {
                if let Err(e) = inner.resolve_self() {
                    return Err(e);
                }
            }
            *i = parser.code_list;
        }
        Ok(())
    }
}
