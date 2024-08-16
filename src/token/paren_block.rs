use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;
use crate::parser::expr_parser::ExprParser;

/// #ParenBlockBranch
/// `()`を使用したプログラムにおけるデータを格納するstruct
/// 中では,
/// - 式を解析する必要がある場合
/// - タイプ宣言を解析する必要がある場合１ ex) (a:T, b:T)
/// - タイプ宣言を解析する必要がある場合２ ex) (T, T)
/// があり個別に呼び出すパーサを実装する必要がある。
/// 実装する
#[derive(Clone, Debug)]
pub struct ParenBlockBranch {
    pub contents: Option<Vec<BaseElem>>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for ParenBlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        // 式パーサーによって解析
        if let Some(a) = &self.contents {
            let mut parser =
                ExprParser::create_parser_from_vec(a.to_vec(), self.depth + 1, self.loopdepth);
            match parser.code2vec() {
                Ok(_) => {
                    let mut rlist = parser.code_list;
                    for i in &mut rlist {
                        i.resolve_self()?
                    }
                    self.contents = Some(rlist);
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}

impl ASTBranch for ParenBlockBranch {
    fn show(&self) {
        println!(
            "{}Paren depth{}\n(",
            " ".repeat(self.depth as usize),
            self.depth
        );
        if let Some(e) = &self.contents {
            for i in e {
                i.show();
            }
        }
        println!("{})", " ".repeat(self.depth as usize));
    }

    fn get_show_as_string(&self) -> String {
        let open_section = format!(
            "{}Paren depth{}\n(",
            " ".repeat(self.depth as usize),
            self.depth
        );
        let mut group_section = String::new();
        if let Some(e) = &self.contents {
            for i in e {
                group_section = format!("{}{}", group_section, i.get_show_as_string());
            }
        }
        let close_section = format!("{})", " ".repeat(self.depth as usize));
        format!("{}{}{}", open_section, group_section, close_section)
    }
}

impl ASTAreaBranch for ParenBlockBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        Self {
            contents,
            depth,
            loopdepth,
        }
    }
}
