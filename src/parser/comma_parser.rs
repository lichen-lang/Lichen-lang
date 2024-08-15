use crate::abs::ast::BaseElem;
use crate::parser::core_parser::Parser;

struct CommaParser {
    pub code: String,
    pub code_list: Vec<BaseElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl CommaParser {}

impl Parser<'_> for CommaParser {
    fn create_parser_from_vec(code_list: Vec<BaseElem>, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: String::new(),
            code_list: code_list,
            depth: depth,
            loopdepth: loopdepth,
        }
    }

    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: code,
            code_list: Vec::new(),
            depth: depth,
            loopdepth: loopdepth,
        }
    }

    fn code2_vec_pre_proc_func(&self, code: &String) -> Vec<BaseElem> {
        todo!()
    }

    fn resolve(&mut self) -> Result<(), crate::errors::parser_errors::ParserError> {
        todo!()
    }
}
