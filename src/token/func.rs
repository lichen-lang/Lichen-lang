use crate::abs::ast::*;
use crate::abs::gen::*;

use crate::errors::generate_errors::GenerateError;
use crate::errors::parser_errors::ParserError;
use crate::parser::comma_parser::CommaParser;
use crate::parser::core_parser::Parser;

use super::paren_block::ParenBlockBranch;

/// # FuncBranch
/// 関数呼び出しのトークン
#[derive(Clone, Debug)]
pub struct FuncBranch {
    pub name: Box<ExprElem>,     // 呼び出している関数の名前
    pub contents: Vec<ExprElem>, // 引数
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for FuncBranch {
    fn show(&self) {
        println!("{}func name", " ".repeat(self.depth as usize * 4));
        self.name.show();
        println!("{}(", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            i.show();
        }
        println!("{})", " ".repeat(self.depth as usize * 4));
    }

    fn get_show_as_string(&self) -> String {
        let function_name_section = format!(
            "{}func name\n{}",
            " ".repeat(self.depth as usize * 4),
            self.name.get_show_as_string()
        );
        let paren_open = format!("{}(\n", " ".repeat(self.depth as usize * 4));
        let mut args_group = String::new();
        for j in &self.contents {
            args_group = format!(
                "{}{}{}\n",
                " ".repeat(self.depth as usize * 4),
                args_group,
                j.get_show_as_string()
            );
        }
        let paren_close = format!("{})", " ".repeat(self.depth as usize * 4));
        format!(
            "{}{}{}{}",
            function_name_section, paren_open, args_group, paren_close
        )
    }
}

impl RecursiveAnalysisElements for FuncBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        self.name.resolve_self()?;
        // 呼び出し元の自己解決
        // 引数の解決
        if self.contents.is_empty() {
            // このresolve_self methodが走る時点で長さが1でなければ不正
            return Err(ParserError::DevError);
        }
        let first_elem = &self.contents[0];

        if let ExprElem::OpeElem(_) = &*self.name {
            // 演算子だった場合はpass
        } else if let ExprElem::ParenBlockElem(ParenBlockBranch {
            contents: v,
            depth,
            loopdepth,
        }) = first_elem
        {
            if 1 < self.contents.len() {
                return Err(ParserError::DevError);
            }
            let mut c_parser = CommaParser::create_parser_from_vec(v.to_vec(), *depth, *loopdepth);
            c_parser.resolve()?;
            self.contents = c_parser.code_list;
        } else {
            return Err(ParserError::DevError);
        }
        for inner in &mut self.contents {
            inner.resolve_self()?;
        }
        Ok(())
    }
}


impl Wasm_gen for FuncBranch {

    fn generate_wasm(&self) -> Result<String, GenerateError> {
        let mut assembly_text:String = String::new();

        // 関数処理部分
        match &*self.name {
            // pass
            ExprElem::OpeElem(ope_b) => {
                // 演算子のとき
                // 必ず２つの引数が渡されるが`-1`などの場合に注意が必要
                assembly_text.push_str(&ope_b.generate_wasm(
                        &self.contents[0],
                        &self.contents[1]
                )?);
            }

            ExprElem::WordElem(word_b) => {
                // 普通の関数のとき
                // 引数処理部分
                for i in &self.contents{
                    match i{
                        ExprElem::ItemElem(b) => {
                            assembly_text.push_str(&b.generate_wasm()?);
                        }
                        _ => {
                            // 必ず引数はアイテムになるのでエラー
                            // ここにitem以外の要素を検知した場合は、
                            // コンパイラの実装に何らかの問題があります
                            return Err(GenerateError::Deverror);
                        }
                    }
                }
                assembly_text.push_str(&format!("call ${}\n", word_b.contents));
            }

            _ => {
                // ここは関数を返すifやwhileを定義しない限りerrorになる
                return Err(GenerateError::Deverror);
            }
        }
        Ok(assembly_text)
    }
}

