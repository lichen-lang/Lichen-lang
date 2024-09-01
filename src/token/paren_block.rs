use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
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

impl ASTAreaBranch for ParenBlockBranch {
    fn new(contents: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self {
        Self {
            contents,
            depth,
            loopdepth,
        }
    }
}
