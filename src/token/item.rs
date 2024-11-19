use crate::abs::ast::*;

use crate::abs::gen::Wasm_gen;
use crate::errors::generate_errors::GenerateError;

use crate::errors::parser_errors::ParserError;
use crate::parser::expr_parser::ExprParser;

/// 引数などの式を格納します
#[derive(Clone, Debug)]
pub struct ItemBranch {
    pub contents: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ItemBranch{
    /// 何も要素を持たない引数だった場合trueを返却する
    pub fn has_no_elem(&self) -> bool {
        self.contents.is_empty()
    }
}

impl ASTBranch for ItemBranch {
    fn show(&self) {
        println!("{}Item(", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            i.show();
        }
        println!("{})", " ".repeat(self.depth as usize * 4));
    }

    fn get_show_as_string(&self) -> String {
        let mut rstr = format!("{}Item(\n", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            rstr = format!("{}{}", rstr, i.get_show_as_string());
        }
        rstr = format!("{}\n{})", rstr, " ".repeat(self.depth as usize * 4));
        rstr
    }
}

impl RecursiveAnalysisElements for ItemBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        // 式パーサーによって解析
        let mut parser =
            ExprParser::create_parser_from_vec(self.contents.clone(), self.depth, self.loopdepth);
        match parser.code2vec() {
            Ok(_) => {
                let mut rlist = parser.code_list;
                for i in &mut rlist {
                    i.resolve_self()?
                }
                self.contents = rlist;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl Wasm_gen for ItemBranch {

    fn generate(&self) -> Result<String, GenerateError> {
        // 複数の場合もあることに注意
        let mut assembly_text = String::default();
        if self.contents.len() == 0{
            // itemの中に何も要素を持たない場合
            // 例えば、考えられるのは'-'だったりする場合
        } else if self.contents.len() == 1 {
            // Itemの中に要素が一つだけの場合
            // （特別に修飾子が付与されない場合）
            // TODO
            // ここでは、変数をi32として扱います
            match &self.contents[0]{
                ExprElem::WordElem(word_b) => {
                    if word_b.self_is_num()? {
                        // もし数字だった場合
                        assembly_text.push_str(&format!("i32.const {}\n", word_b.contents));
                    } else {
                        // もし何らかの変数だった場合
                        assembly_text.push_str(&format!("local.get ${}\n", word_b.contents));
                    }
                }

                ExprElem::FuncElem(func_b) => {
                    assembly_text.push_str(&func_b.generate()?);
                }

                ExprElem::ParenBlockElem(paren_b) => {
                    assembly_text.push_str(&paren_b.generate()?);
                }
                _ => {
                    return Err(GenerateError::Deverror);
                }
            }
        } else {
            // ここは例えば、let mut aなどの場合
            // 
            // borrow mut &mut とか引数に
            return Err(GenerateError::Deverror);// 未実装
        }
        return Ok(assembly_text);
    }
}
