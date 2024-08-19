use crate::abs::ast::ASTAreaBranch;
use crate::abs::ast::BaseElem;

use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;

use crate::token::string::StringBranch;

pub struct CommaParser {
    pub code: String,
    pub code_list: Vec<BaseElem>,
    pub out_code_list: Vec<Vec<BaseElem>>,
    pub depth: isize,
    pub loopdepth: isize,
}

/// This parser is a bit anomalous.
/// It performs specialized parsing of the argument part of a function that takes multiple arguments.
impl CommaParser {
    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        self.grouping_quotation()?;
        // grouping_elements
        self.grouping_elements(
            BaseElem::BlockElem,
            Self::BLOCK_BRACE_OPEN,  // {
            Self::BLOCK_BRACE_CLOSE, // }
        )?;
        self.grouping_elements(
            BaseElem::ListBlockElem,
            Self::BLOCK_LIST_OPEN,  // [
            Self::BLOCK_LIST_CLOSE, // ]
        )?;
        self.grouping_elements(
            BaseElem::ParenBlockElem,
            Self::BLOCK_PAREN_OPEN,  // (
            Self::BLOCK_PAREN_CLOSE, // )
        )?;
        self.grouping_args()?;
        Ok(())
    }

    fn grouping_args(&mut self) -> Result<(), ParserError> {
        let mut group: Vec<BaseElem> = vec![];
        for inner in &self.code_list {
            if let BaseElem::UnKnownElem(v) = inner {
                if v.contents == Self::COMMA {
                    self.out_code_list.push(group.clone());
                    group.clear();
                } else {
                    group.push(inner.clone());
                }
            } else {
                group.push(inner.clone());
            }
        }
        if !group.is_empty() {
            self.out_code_list.push(group.clone());
        }
        Ok(())
    }
    fn grouping_quotation(&mut self) -> Result<(), ParserError> {
        let mut open_flag = false;
        let mut escape_flag = false;
        let mut rlist = Vec::new();
        let mut group = String::new();

        for inner in &self.code_list {
            if let BaseElem::UnKnownElem(ref v) = inner {
                if escape_flag {
                    group.push(v.contents);
                    escape_flag = false
                } else if v.contents == '"'
                // is quochar
                {
                    if open_flag {
                        group.push(v.contents);
                        rlist.push(BaseElem::StringElem(StringBranch {
                            contents: group.clone(),
                            depth: self.depth,
                        }));
                        group.clear();
                        open_flag = false;
                    } else {
                        group.push(v.contents);
                        open_flag = true;
                    }
                } else if open_flag {
                    escape_flag = v.contents == '\\';
                    group.push(v.contents);
                } else {
                    rlist.push(inner.clone());
                }
            } else {
                rlist.push(inner.clone());
            }
        }
        if open_flag {
            return Err(ParserError::QuotationNotClosed);
        }
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_elements<T>(
        &mut self,
        elemtype: fn(T) -> BaseElem,
        open_char: char,
        close_char: char,
    ) -> Result<(), ParserError>
    where
        T: ASTAreaBranch,
    {
        let mut rlist: Vec<BaseElem> = Vec::new();
        let mut group: Vec<BaseElem> = Vec::new();
        let mut depth: isize = 0;

        for inner in &self.code_list {
            if let BaseElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    match depth {
                        0 => { /*pass*/ }
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::Uncategorized),
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    match depth {
                        0 => {
                            rlist.push(elemtype(ASTAreaBranch::new(
                                Some(group.clone()),
                                self.depth,
                                self.loopdepth,
                            )));
                            group.clear();
                        }
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::Uncategorized),
                    }
                } else {
                    match depth {
                        0 => rlist.push(inner.clone()),
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::Uncategorized),
                    }
                }
            } else {
                match depth {
                    0 => rlist.push(inner.clone()),
                    1.. => group.push(inner.clone()),
                    _ => return Err(ParserError::BraceNotClosed),
                }
            }
        }
        if depth != 0 {
            return Err(ParserError::BraceNotClosed);
        }
        self.code_list = rlist;
        Ok(())
    }
}

impl Parser<'_> for CommaParser {
    fn create_parser_from_vec(code_list: Vec<BaseElem>, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: String::new(),
            code_list,
            out_code_list: Vec::new(),
            depth,
            loopdepth,
        }
    }

    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code,
            code_list: Vec::new(),
            out_code_list: Vec::new(),
            depth,
            loopdepth,
        }
    }

    fn code2_vec_pre_proc_func(&self, code: &str) -> Vec<BaseElem> {
        todo!()
    }

    fn resolve(&mut self) -> Result<(), ParserError> {
        self.code2vec()?;
        Ok(())
    }
}
