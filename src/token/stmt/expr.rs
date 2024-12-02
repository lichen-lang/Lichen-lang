use crate::abs::ast::{ASTBranch, ExprElem, RecursiveAnalysisElements, Token};
use crate::abs::gen::Wasm_gen;
use crate::errors::generate_errors::GenerateError;
use crate::errors::parser_errors::ParserError;
use crate::parser::expr_parser::ExprParser;

/// 式の集合を扱います
#[derive(Clone, Debug)]
pub struct ExprBranch {
    pub code_list: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for ExprBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        let mut parser =
            ExprParser::create_parser_from_vec(self.code_list.clone(), self.depth, self.loopdepth);
        match parser.code2vec() {
            Ok(_) => {
                let mut rlist = parser.code_list;
                for i in &mut rlist {
                    i.resolve_self()?;
                }
                self.code_list = rlist;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl ASTBranch for ExprBranch {
    fn show(&self) {
        //pass
        println!("ExprBranch({}){{", self.code_list.len());
        for i in &self.code_list {
            i.show();
        }
        println!("}}");
    }

    fn get_show_as_string(&self) -> String {
        //pass
        let mut rlist: String = format!("ExprBranch({}){{\n", self.code_list.len());
        for i in &self.code_list {
            rlist.push_str(&" ".repeat(4 * (self.depth as usize) + 1));
            rlist.push_str(&i.get_show_as_string());
        }
        rlist.push_str("}\n");
        rlist
    }
}

impl Wasm_gen for ExprBranch {
    fn generate_wasm(&self) -> Result<String, GenerateError> {
        let mut assembly_text = String::new();
        for expr in &self.code_list {
            match expr {
                ExprElem::FuncElem(func_b) => {
                    // 普通の式の場合
                    assembly_text.push_str(&func_b.generate_wasm()?);
                }
                ExprElem::SyntaxBoxElem(synt_b) => {
                    assembly_text.push_str(&synt_b.generate_wasm()?);
                }
                _ => {
                    // ここではエラーを返すべきである
                }
            }
            //assembly_text.push_str();
        }
        Ok(assembly_text)
    }
}
