use crate::abs::ast::*;

#[derive(Clone, Debug)]
pub struct CommentBranch {
    pub contents: String,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for CommentBranch {
    fn show(&self) {
        println!("comment \"{}\"", self.contents)
    }

    fn get_show_as_string(&self) -> String {
        format!("comment \"{}\"", self.contents)
    }
}
