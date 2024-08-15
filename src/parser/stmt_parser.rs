use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::*;
use crate::token::string::StringBranch;
use crate::token::word::WordBranch;

pub struct StmtParser {
    // TODO: 一時的にpublicにしているだけ
    pub code: String,
    pub code_list: Vec<BaseElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl StmtParser {
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
                } else if v.contents == Self::DOUBLE_QUOTATION
                // '"'
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
                    escape_flag = v.contents == Self::ESCAPECHAR;
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
                    if depth > 0 {
                        group.push(inner.clone());
                    } else if depth == 0 {
                        // pass
                    } else {
                        return Err(ParserError::Uncategorized);
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    if depth > 0 {
                        group.push(inner.clone());
                    } else if depth == 0 {
                        rlist.push(elemtype(ASTAreaBranch::new(
                            Some(group.clone()),
                            self.depth,
                            self.loopdepth,
                        )));
                        group.clear();
                    } else {
                        return Err(ParserError::Uncategorized);
                    }
                } else {
                    if depth > 0 {
                        group.push(inner.clone());
                    } else if depth == 0 {
                        rlist.push(inner.clone());
                    } else {
                        return Err(ParserError::Uncategorized);
                    }
                }
            } else {
                if depth > 0 {
                    group.push(inner.clone());
                } else if depth == 0 {
                    rlist.push(inner.clone());
                } else {
                    return Err(ParserError::BraceNotClosed);
                }
            }
        }
        if depth != 0 {
            return Err(ParserError::BraceNotClosed);
        }
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_words(&mut self) -> Result<(), ParserError> {
        let mut rlist: Vec<BaseElem> = Vec::new();
        let mut group: String = String::new();

        for inner in &self.code_list {
            if let BaseElem::UnKnownElem(ref e) = inner {
                if Self::SPLIT_CHAR.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        rlist.push(BaseElem::WordElem(WordBranch {
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                    }
                } else if Self::EXCLUDE_WORDS.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        rlist.push(BaseElem::WordElem(WordBranch {
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                    }
                    rlist.push(inner.clone());
                } else {
                    group.push(e.contents);
                }
            } else {
                if !group.is_empty() {
                    rlist.push(BaseElem::WordElem(WordBranch {
                        contents: group.clone(),
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    }));
                    group.clear();
                }
                rlist.push(inner.clone());
            }
        }
        if !group.is_empty() {
            rlist.push(BaseElem::WordElem(WordBranch {
                contents: group.clone(),
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
            group.clear();
        }
        self.code_list = rlist;
        Ok(())
    }

    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        self.grouping_quotation()?;
        self.grouping_words()?;
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
        Ok(())
    }
}

impl Parser<'_> for StmtParser {
    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code,
            code_list: Vec::new(),
            depth,
            loopdepth,
        }
    }

    fn create_parser_from_vec(code_list: Vec<BaseElem>, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: String::new(),
            code_list,
            depth,
            loopdepth,
        }
    }
    fn resolve(&mut self) -> Result<(), ParserError> {
        self.code_list = self.code2_vec_pre_proc_func(&self.code);
        if let Err(e) = self.code2vec() {
            Err(e)
        } else {
            for i in &mut self.code_list {
                i.resolve_self()?;
            }
            Ok(())
        }
    }
}
