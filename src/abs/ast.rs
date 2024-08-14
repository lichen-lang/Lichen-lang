use crate::token::{
    block::BlockBranch, func::FuncBranch, list_block::ListBlockBranch, operator::OperatorBranch,
    paren_block::ParenBlockBranch, string::StringBranch, syntax::SyntaxBranch,
    syntax_box::SyntaxBoxBranch, unknown::UnKnownBranch, word::WordBranch,
};

use crate::errors::parser_errors::ParserError;

/// # BaseElem
/// 抽象的なtoken
/// プログラムの要素を表現できる
#[derive(Clone)]
pub enum BaseElem {
    BlockElem(BlockBranch),
    ListBlockElem(ListBlockBranch),
    ParenBlockElem(ParenBlockBranch),
    SyntaxElem(SyntaxBranch),
    SyntaxBoxElem(SyntaxBoxBranch),
    FuncElem(FuncBranch),
    // without ASTAreaBranch trait structures
    StringElem(StringBranch),
    WordElem(WordBranch),
    OpeElem(OperatorBranch),
    UnKnownElem(UnKnownBranch),
}

impl BaseElem {
    pub fn show(&self) {
        match self {
            Self::BlockElem(e) => e.show(),
            Self::UnKnownElem(e) => e.show(),
            Self::StringElem(e) => e.show(),
            Self::ListBlockElem(e) => e.show(),
            Self::ParenBlockElem(e) => e.show(),
            Self::WordElem(e) => e.show(),
            Self::SyntaxElem(e) => e.show(),
            Self::SyntaxBoxElem(e) => e.show(),
            Self::FuncElem(e) => e.show(),
            Self::OpeElem(e) => e.show(),
        }
    }

    pub fn get_show_as_string(&self) -> String {
        match self {
            Self::BlockElem(e) => e.get_show_as_string(),
            Self::UnKnownElem(e) => e.get_show_as_string(),
            Self::StringElem(e) => e.get_show_as_string(),
            Self::ListBlockElem(e) => e.get_show_as_string(),
            Self::ParenBlockElem(e) => e.get_show_as_string(),
            Self::WordElem(e) => e.get_show_as_string(),
            Self::SyntaxElem(e) => e.get_show_as_string(),
            Self::SyntaxBoxElem(e) => e.get_show_as_string(),
            Self::FuncElem(e) => e.get_show_as_string(),
            Self::OpeElem(e) => e.get_show_as_string(),
        }
    }
    pub fn resolve_self(&mut self) -> Result<(), ParserError> {
        match self {
            // recursive analysis elements
            Self::BlockElem(e) => return e.resolve_self(),
            Self::ListBlockElem(e) => return e.resolve_self(),
            Self::ParenBlockElem(e) => return e.resolve_self(),
            Self::SyntaxElem(e) => return e.resolve_self(),
            Self::SyntaxBoxElem(e) => return e.resolve_self(),
            Self::FuncElem(e) => return e.resolve_self(),

            // unrecursive analysis elements
            Self::StringElem(_) => return Ok(()),
            Self::WordElem(_) => return Ok(()),
            Self::OpeElem(_) => return Ok(()),
            Self::UnKnownElem(_) => return Ok(()),
        }
    }
}

/// #  ASTBranch
/// このtraitを実装している構造体は
/// - 自分自身の構造をわかりやすく出力できる
pub trait ASTBranch {
    fn show(&self);
    fn get_show_as_string(&self) -> String;
}

/// # ASTAreaBranch
/// ## resolve_self
/// depthをインクリメントするときは、`resolve_self`内で宣言するParserにself.get_depth + 1をして実装する必要がある
pub trait ASTAreaBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self;
    // fn resolve_self(&mut self) -> Result<&str, String>;
}

pub trait RecursiveAnalysisElements {
    fn resolve_self(&mut self) -> Result<(), ParserError>;
}
