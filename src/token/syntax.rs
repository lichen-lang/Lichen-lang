use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::token::{block::BlockBranch, paren_block::ParenBlockBranch};

/// # SyntaxBranch
/// `if` `elif` `else` `while` `loop` `for`などのデータを扱うstruct
/// resolve_selfはそれぞれ
/// `()`で格納されているデータに関しては`ParenBlockBranch`をnormalで呼び出す
/// `{}`で格納されているデータに関しては`BlockBranch`のパーサに丸投げする。
/// 当然、全てのブロックが何かで満たされるわけではないので注意
#[cfg(debug_assertions)]
#[derive(Clone, Debug)]
pub struct SyntaxBranch {
    pub name: String,
    pub expr: Option<ParenBlockBranch>,
    pub contents: Option<BlockBranch>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for SyntaxBranch {
    fn show(&self) {
        todo!()
    }
    fn get_show_as_string(&self) -> String {
        todo!()
    }
}

impl RecursiveAnalysisElements for SyntaxBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}
