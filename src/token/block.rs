use crate::abs::ast::*;

use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;
use crate::parser::stmt_parser::*;

/// # BlockBranch
/// ブロックを格納するデータstruct
/// 内部では文を解析するパーサを呼び出す必要がある
///
#[derive(Clone, Debug)]
pub struct BlockBranch {
    pub contents: Option<Vec<BaseElem>>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl RecursiveAnalysisElements for BlockBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        if let Some(a) = &self.contents {
            // let parser = StateParser::new(String::from(""), self.depth + 1, self.loopdepth);
            let mut parser =
                StmtParser::create_parser_from_vec(a.to_vec(), self.depth + 1, self.loopdepth);
            match parser.code2vec() {
                Ok(_) => {
                    let mut rlist = parser.code_list;
                    for i in &mut rlist {
                        // if let Err(e) = i.resolve_self() {
                        //     return Err(e);
                        // }
                        i.resolve_self()?;
                    }
                    self.contents = Some(rlist);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }
}

impl ASTAreaBranch for BlockBranch {
    fn new(contents: Option<Vec<BaseElem>>, depth: isize, loopdepth: isize) -> Self {
        Self {
            contents,
            depth,
            loopdepth,
        }
    }
}

impl ASTBranch for BlockBranch {
    fn show(&self) {
        println!(
            "{}BlockBranch depth{} (",
            " ".repeat(self.depth as usize),
            self.depth
        );
        if let Some(e) = &self.contents {
            for i in e {
                i.show();
            }
        }
        println!(")");
    }

    fn get_show_as_string(&self) -> String {
        let mut show_group = String::new();
        if let Some(e) = &self.contents {
            for i in e {
                show_group = format!("{}{}", show_group, i.get_show_as_string());
            }
        }
        format!(
            "{}BlockBranch depth{} (\n{}",
            " ".repeat(self.depth as usize),
            self.depth,
            show_group
        )
    }
}
