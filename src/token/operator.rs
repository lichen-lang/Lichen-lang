use crate::abs::ast::*;
/// #OperatorBranch
/// 全ての演算子
#[derive(Clone, Debug)]
pub struct OperatorBranch {
    pub ope: String,
    pub depth: isize,
}

impl ASTBranch for OperatorBranch {
    fn show(&self) {
        println!(
            "{}Operator({})",
            " ".repeat(self.depth as usize * 4),
            self.ope
        );
    }

    fn get_show_as_string(&self) -> String {
        format!(
            "{}Operator({})",
            " ".repeat(self.depth as usize * 4),
            self.ope
        )
    }
}
