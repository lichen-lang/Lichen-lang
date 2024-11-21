use crate::abs::ast::*;
use crate::abs::gen::Wasm_gen;
use crate::errors::parser_errors::ParserError;
use crate::parser::expr_parser::ExprParser;
use crate::errors::generate_errors::GenerateError;

/// #ParenBlockBranch
/// `()`を使用したプログラムにおけるデータを格納するstruct
/// 中では,
/// - 式を解析する必要がある場合
/// - タイプ宣言を解析する必要がある場合２ ex) (T, T)
#[derive(Clone, Debug)]
pub struct ParenBlockBranch {
    pub contents: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for ParenBlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        // 式パーサーによって解析
        let mut parser = ExprParser::create_parser_from_vec(
            self.contents.clone(),
            self.depth + 1,
            self.loopdepth,
        );
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

impl ASTBranch for ParenBlockBranch {
    fn show(&self) {
        println!("{}Paren\n(", " ".repeat(self.depth as usize));
        for i in &self.contents {
            i.show();
        }
        println!("{})", " ".repeat(self.depth as usize));
    }

    fn get_show_as_string(&self) -> String {
        let open_section = format!("{}Paren\n(", " ".repeat(self.depth as usize));
        let mut group_section = String::new();
        for i in &self.contents {
            group_section = format!("{}{}", group_section, i.get_show_as_string());
        }
        let close_section = format!("{})", " ".repeat(self.depth as usize));
        format!("{}{}{}", open_section, group_section, close_section)
    }
}

impl ASTAreaBranch<ExprElem> for ParenBlockBranch {
    fn new(contents: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self {
        Self {
            contents,
            depth,
            loopdepth,
        }
    }
}


impl Wasm_gen for ParenBlockBranch{
    fn generate_wasm(&self) -> Result<String, crate::errors::generate_errors::GenerateError> {
        let mut assembly_text = String::default();
        match self.contents.len(){
            0 => {
                // pass
                // カッコの中に何もない場合
                // ()
                // ただ、ここではは想定されていないエラーをキャッチする必要がある
                // ```
                // (()) / a
                // ```
            }

            1 => {
                match &self.contents[0]{
                    // example `(1+a)`
                    ExprElem::FuncElem(func_b) => {
                        assembly_text.push_str(&func_b.generate_wasm()?);
                    }

                    // example `(a)`
                    ExprElem::WordElem(word_b) => {
                        if word_b.self_is_num()? {
                            assembly_text.push_str(&format!("i32.const {}\n", word_b.contents));
                        } else {
                            // もし何らかの変数だった場合
                            assembly_text.push_str(&format!("local.get ${}\n", word_b.contents));
                        }
                    }

                    // `((a + 1))`
                    ExprElem::ParenBlockElem(paren_b) => {
                        assembly_text.push_str(&paren_b.generate_wasm()?);
                    }

                    _ => {
                        // pass
                        return Err(GenerateError::Deverror);
                    }
                }

            }

            _ => {
                // 対応していない
                // `(,,,)`的なシチュエーション
                return Err(GenerateError::Deverror);
            }
        }
        Ok(assembly_text)
    }
}

