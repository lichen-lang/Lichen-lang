use crate::abs::ast::{ASTBranch, ExprElem, RecursiveAnalysisElements, StmtElem, Token};
use crate::abs::gen::Wasm_gen;
use crate::errors::parser_errors::ParserError;
use crate::parser::expr_parser::ExprParser;
use crate::errors::generate_errors::GenerateError;


/// `return` `continue` `break` `yield` `let`
/// などを処理をする
/// 
/// breakやcontinueなどの処理は、
/// 外側のループ構造によってジャンプする先を
/// 常に読み替える必要がある
#[derive(Clone, Debug)]
pub struct StmtBranch {
    pub head:String,
    pub code_list: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}


impl RecursiveAnalysisElements for StmtBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        let mut parser = ExprParser::create_parser_from_vec(
            self.code_list.clone(),
            self.depth,
            self.loopdepth);
        match parser.code2vec(){
            Ok(_) => {
                let mut rlist = parser.code_list;
                for i in &mut rlist {
                    // pass
                    i.resolve_self()?;
                }
                self.code_list = rlist;
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}


impl ASTBranch for StmtBranch {
    fn show(&self) {
        println!("control Branch {}", self.head);
        for inner in &self.code_list{
            inner.show();
        }
    }

    fn get_show_as_string(&self) -> String {
        let mut rtext = String::default();
        rtext.push_str(&format!("control Branch \"{}\" (\n", self.head));
        for inner in &self.code_list{
            rtext.push_str(&inner.get_show_as_string());
        }
        rtext.push_str(")\n");
        return rtext;
    }
}


impl Wasm_gen for StmtBranch {
    fn generate_wasm(&self) -> Result<String, GenerateError> {
        let mut assembly_text = String::default();

        match &*self.head {
            "return" => {
                assembly_text.push_str("return\n");
            }
            "break" => {
                // ここは、どのループの入れ子構造に属しているかで変わる
            }
            "continue" => {
                // ここは、どのループの入れ子構造に属しているかで変わる
            }
            _ => {
                // error 不明なcontroll statement
                //
            }

        }
        Ok(assembly_text)
    }
}

