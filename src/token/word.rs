use crate::abs::ast::*;

/// # WordBranch
/// 単語を格納するためのstruct
/// ASTAreaBranchを実装しないため`resolve_self`メソッドを持たない

#[derive(Clone, Debug)]
pub struct WordBranch {
    pub contents: String,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for WordBranch {
    fn show(&self) {
        println!(
            "{}Word \"{}\"",
            " ".repeat(self.depth as usize * 4),
            self.contents
        )
    }

    fn get_show_as_string(&self) -> String {
        format!(
            "{}Word \"{}\"",
            " ".repeat(self.depth as usize * 4),
            self.contents
        )
    }
}
