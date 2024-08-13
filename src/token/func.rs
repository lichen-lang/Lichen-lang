use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
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
        println!("func name");
        self.name.show();
        println!("(");
        for (i, j) in self.contents.iter().enumerate() {
            print!("arg{}\n", i);
            for k in j {
                k.show();
            }
        }
        println!(")");
    }
}

impl RecursiveAnalysisElements for FuncBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        if let Err(e) = self.name.resolve_self() {
            return Err(e);
        } // 呼び出し元の自己解決
        for i in &mut self.contents {
            for j in i {
                if let Err(e) = j.resolve_self() {
                    return Err(e);
                } // 呼び出し元の自己解決
            }
        }
        Ok(())
    }
}
