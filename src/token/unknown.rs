use crate::abs::ast::*;

/// # UnKnownBranch
///未定トークンが以下のstructに分類される
#[cfg(debug_assertions)]
#[derive(Clone, Debug)]
pub struct UnKnownBranch {
    pub contents: char,
}

impl ASTBranch for UnKnownBranch {
    fn show(&self) {
        println!("UnKnownBranch :\"{}\"", self.contents);
    }
    fn get_show_as_string(&self) -> String {
        format!("UnKnownBranch :\"{}\"", self.contents)
    }
}
