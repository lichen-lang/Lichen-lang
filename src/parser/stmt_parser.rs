use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::*;

use crate::token::operator::OperatorBranch;
use crate::token::stmt::expr::ExprBranch;
use crate::token::string::StringBranch;
use crate::token::word::WordBranch;

/// # StmtParser
pub struct StmtParser {
    pub code: String,
    pub code_list: Vec<StmtElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl StmtParser {
    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        self.grouping_quotation()?;
        self.grouping_elements(
            StmtElem::BlockElem,
            Self::BLOCK_BRACE_OPEN,  // {
            Self::BLOCK_BRACE_CLOSE, // }
        )?;
        self.grouping_elements(
            StmtElem::ListBlockElem,
            Self::BLOCK_LIST_OPEN,  // [
            Self::BLOCK_LIST_CLOSE, // ]
        )?;
        self.grouping_elements(
            StmtElem::ParenBlockElem,
            Self::BLOCK_PAREN_OPEN,  // (
            Self::BLOCK_PAREN_CLOSE, // )
        )?;
        self.grouping_words()?;
        self.split_semicolon()?;
        Ok(())
    }

    fn grouping_quotation(&mut self) -> Result<(), ParserError> {
        let mut open_flag = false;
        let mut escape_flag = false;
        let mut rlist = Vec::new();
        let mut group = String::new();

        for inner in &self.code_list {
            if let StmtElem::UnKnownElem(ref v) = inner {
                if escape_flag {
                    group.push(v.contents);
                    escape_flag = false
                } else if v.contents == Self::DOUBLE_QUOTATION
                // is quochar
                {
                    if open_flag {
                        group.push(v.contents);
                        rlist.push(StmtElem::StringElem(StringBranch {
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
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

    fn grouping_elements<T, U>(
        &mut self,
        elemtype: fn(T) -> StmtElem,
        open_char: char,
        close_char: char,
    ) -> Result<(), ParserError>
    where
        T: ASTAreaBranch<U>,
        U: Clone + Token + ProcToken, // ExprElem or StmtElem
    {
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut group: Vec<U> = Vec::new();
        let mut depth: isize = 0;

        for inner in &self.code_list {
            if let StmtElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    match depth {
                        0 => { /*pass*/ }
                        1.. => group.push(Token::set_char_as_unknown(b.contents)),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    match depth {
                        0 => {
                            rlist.push(elemtype(ASTAreaBranch::<U>::new(
                                group.clone(),
                                self.depth,
                                self.loopdepth,
                            )));
                            group.clear();
                        }
                        1.. => group.push(Token::set_char_as_unknown(b.contents)),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                } else {
                    match depth {
                        0 => rlist.push(inner.clone()),
                        1.. => group.push(Token::set_char_as_unknown(b.contents)),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                }
            } else {
                match depth {
                    0 => rlist.push(inner.clone()),
                    1.. => {
                        match &inner {
                            StmtElem::StringElem(s) => {
                                group.push(ProcToken::t_string(
                                    s.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            StmtElem::BlockElem(bl) => {
                                group.push(ProcToken::t_block(
                                    bl.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            StmtElem::ParenBlockElem(pb) => {
                                group.push(ProcToken::t_parenblock(
                                    pb.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            StmtElem::ListBlockElem(lb) => {
                                group.push(ProcToken::t_listblock(
                                    lb.contents.clone(),
                                    self.depth,
                                    self.loopdepth,
                                ));
                            }
                            // todo
                            _ => {
                                // todo error処理
                                return Err(ParserError::UnexpectedType);
                            }
                        }
                    }
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

    fn grouping_words(&mut self) -> Result<(), ParserError> {
        // macro
        macro_rules! add_rlist {
            ($rlist:expr,$group:expr) => {
                if let Ok(_) = Self::find_ope_priority(&$group) {
                    $rlist.push(StmtElem::OpeElem(OperatorBranch {
                        ope: $group.clone(),
                        depth: self.depth,
                    }))
                } else {
                    $rlist.push(StmtElem::WordElem(WordBranch {
                        contents: $group.clone(),
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    }));
                }
            };
        }
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut group: String = String::new();
        let ope_str = Self::LENGTH_ORDER_OPE_LIST.map(|a| a.opestr).join("");

        for inner in &self.code_list {
            if let StmtElem::UnKnownElem(ref e) = inner {
                if Self::SPLIT_CHAR.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        add_rlist!(rlist, group);
                        group.clear();
                    }
                } else if Self::EXCLUDE_WORDS.contains(&e.contents) || ope_str.contains(e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        add_rlist!(rlist, group);
                        group.clear();
                    }
                    rlist.push(inner.clone());
                } else {
                    group.push(e.contents);
                }
            } else {
                if !group.is_empty() {
                    add_rlist!(rlist, group);
                    group.clear();
                }
                rlist.push(inner.clone());
            }
        }
        if !group.is_empty() {
            add_rlist!(rlist, group);
            group.clear();
        }
        self.code_list = rlist;
        Ok(())
    }

    pub fn split_semicolon(&mut self) -> Result<(), ParserError> {
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut group: Vec<StmtElem> = Vec::new();
        for inner in &self.code_list {
            match &inner {
                StmtElem::UnKnownElem(unb) => {
                    if unb.contents == Self::SEMICOLON {
                        rlist.push(StmtElem::ExprElem(ExprBranch {
                            code_list: Self::stmt2expr(group.clone())?,
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                    } else {
                        group.push(inner.clone());
                    }
                }
                _ => {
                    group.push(inner.clone());
                }
            }
        }
        if !group.is_empty() {
            rlist.push(StmtElem::ExprElem(ExprBranch {
                code_list: Self::stmt2expr(group.clone())?,
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        }
        self.code_list = rlist;
        Ok(())
    }

    fn stmt2expr(i: Vec<StmtElem>) -> Result<Vec<ExprElem>, ParserError> {
        let mut rlist: Vec<ExprElem> = Vec::new();
        for inner in i {
            rlist.push(match inner {
                StmtElem::StringElem(a) => ExprElem::StringElem(a),
                StmtElem::BlockElem(a) => ExprElem::BlockElem(a),
                StmtElem::ListBlockElem(a) => ExprElem::ListBlockElem(a),
                StmtElem::ParenBlockElem(a) => ExprElem::ParenBlockElem(a),
                StmtElem::OpeElem(a) => ExprElem::OpeElem(a),
                StmtElem::WordElem(a) => ExprElem::WordElem(a),
                StmtElem::UnKnownElem(a) => ExprElem::UnKnownElem(a),
                _ => return Err(ParserError::UnableToConvertType),
            });
        }
        Ok(rlist)
    }

    pub fn create_parser_from_vec(
        code_list: Vec<StmtElem>,
        depth: isize,
        loopdepth: isize,
    ) -> Self {
        Self {
            code: String::new(),
            code_list,
            depth,
            loopdepth,
        }
    }
}

impl Parser<'_> for StmtParser {
    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: code.clone(),
            code_list: Self::code2_vec_pre_proc_func(&code),
            depth,
            loopdepth,
        }
    }

    fn resolve(&mut self) -> Result<(), ParserError> {
        self.code2vec()?;
        for i in &mut self.code_list {
            i.resolve_self()?;
        }
        Ok(())
    }
}
