use crate::abs::ast::*;
use crate::parser::core_parser::*;

pub struct TypeParser {
    pub code: String,
    pub code_list: Vec<ExprElem>,
    pub depth: isize,
    pub loopdepth: isize,
}
