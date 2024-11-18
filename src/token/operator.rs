use std::fmt::format;

use crate::abs::ast::*;
use crate::abs::gen::Wasm_gen;
use crate::errors::generate_errors::GenerateError;

/// #OperatorBranch
/// 全ての演算子
///
#[derive(Clone, Debug)]
pub struct OperatorBranch {
    pub ope: String,
    pub depth: isize,
}

impl ASTBranch for OperatorBranch {
    fn show(&self) {
        println!(
            "{}Operator({})\n",
            " ".repeat(self.depth as usize * 4),
            self.ope
        );
    }

    fn get_show_as_string(&self) -> String {
        format!(
            "{}Operator({})\n",
            " ".repeat(self.depth as usize * 4),
            self.ope
        )
    }
}

impl Wasm_gen for OperatorBranch {
    fn generate(&self) -> Result<String, GenerateError>{
        // 最初はi32向のみ対応
        // ここで送られて来るデータが本当に文字列でいいのか考える
        match &*self.ope{
            // pass
            "+" => {
                Ok(format!("i32.add\n"))
            }
            "-" => {
                Ok(format!("i32.sub\n"))
            }
            "*" => {
                Ok(format!("i32.mul\n"))
            }
            "/" => {
                Ok(format!("i32.div\n"))
            }
            _ => {
                Err(GenerateError::InvalidOperation)
            }
        }
    }
}
