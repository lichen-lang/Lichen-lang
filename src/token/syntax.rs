use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;
use crate::parser::expr_parser::ExprParser;
use crate::parser::stmt_parser::StmtParser;

/// # SyntaxBranch
/// `if` `elif` `else` `while` `loop` `for`などのデータを扱うstruct
/// resolve_selfはそれぞれ
/// `()`で格納されているデータに関しては`ParenBlockBranch`をnormalで呼び出す
/// `{}`で格納されているデータに関しては`BlockBranch`のパーサに丸投げする。
/// 当然、全てのブロックが何かで満たされるわけではないので注意
#[derive(Clone, Debug)]
pub struct SyntaxBranch {
    pub name: String,
    pub expr: Vec<ExprElem>,
    pub contents: Vec<StmtElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for SyntaxBranch {
    fn show(&self) {
        println!("{}", self.name);
        println!("expr");
        for i in &self.expr {
            i.show()
        }
        println!("{}{{", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            i.show()
        }
        println!("{}}}", " ".repeat(self.depth as usize * 4));
    }

    fn get_show_as_string(&self) -> String {
        let mut expr_string = String::new();

        for i in &self.expr {
            expr_string = format!("{}{}", expr_string, i.get_show_as_string());
        }
        expr_string = format!("expr({})", expr_string);

        let mut block_string = format!("{}{{", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            block_string = format!("{}{}\n", block_string, i.get_show_as_string())
        }
        block_string = format!("{}{}}}", block_string, " ".repeat(self.depth as usize * 4));
        format!("{}{}{}", self.name, expr_string, block_string)
    }
}

impl RecursiveAnalysisElements for SyntaxBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        let mut e_parser =
            ExprParser::create_parser_from_vec(self.expr.clone(), self.depth, self.loopdepth);
        e_parser.resolve()?;
        self.expr = e_parser.code_list;
        let mut s_parser =
            StmtParser::create_parser_from_vec(self.contents.clone(), self.depth, self.loopdepth);
        s_parser.resolve()?;
        self.contents = s_parser.code_list;
        Ok(())
    }
}
