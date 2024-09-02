use crate::abs::ast::*;

#[derive(Clone, Debug)]
pub struct TypeBlockBranch {
    pub code_list: Vec<TypeElem>,
    pub depth: isize,
    // loopdepth: isize,
}

impl TypeBlockBranch {}

impl ASTBranch for TypeBlockBranch {
    fn get_show_as_string(&self) -> String {
        let mut rstr = String::new();
        for i in &self.code_list {
            rstr = format!("{}{}", rstr, i.get_show_as_string());
        }
        format!("<{}>", rstr)
    }

    fn show(&self) {
        println!("{}", self.get_show_as_string())
    }
}

impl TypeAreaBranch for TypeBlockBranch {
    fn new(contents: Vec<TypeElem>, depth: isize) -> Self {
        todo!()
    }
}
