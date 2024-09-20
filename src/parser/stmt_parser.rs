use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::*;

use crate::token::comment::CommentBranch;
use crate::token::operator::OperatorBranch;
use crate::token::stmt::stmt::StmtBranch;
use crate::token::string::StringBranch;
use crate::token::word::WordBranch;

enum StringAreaState {
    CommentOpen,  // /*
    CommentStart, // //
    QuotationOpen,
    Closed,
}

/// # StmtParser
pub struct StmtParser {
    pub code: String,
    pub code_list: Vec<StmtElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl StmtParser {
    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        self.grouping_string()?;

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

        // split semicolon
        self.semicolon_split()?;
        // at first input data convert to ast by expr parser
        // let mut expr_parser =
        //     ExprParser::create_parser_from_vec(self.code_list.clone(), self.depth, self.loopdepth);
        // expr_parser.code2vec()?;
        // self.code_list = expr_parser.code_list;
        Ok(())
    }

    fn grouping_string(&mut self) -> Result<(), ParserError> {
        let mut group: String = String::new();
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut open_status: StringAreaState = StringAreaState::Closed;
        let mut ignore_flag = false;
        let mut string_escape_flag = false;

        for (count, inner) in self.code_list.iter().enumerate() {
            if ignore_flag {
                ignore_flag = false;
                // 二文字の判別
                continue;
            }
            if let StmtElem::UnKnownElem(e) = inner {
                match open_status {
                    StringAreaState::CommentStart => {
                        // //が開いているとき
                        if e.contents == '\n' {
                            rlist.push(StmtElem::CommentElem(CommentBranch {
                                contents: group.clone(),
                                depth: self.depth,
                                loopdepth: self.loopdepth,
                            }));
                            open_status = StringAreaState::Closed;
                            group.clear();
                        } else {
                            group.push(e.contents);
                        }
                    }
                    StringAreaState::CommentOpen => {
                        // /*が開いているとき
                        if Self::COMMENT_CLOSE.starts_with(e.contents)
                        // "*" == e.content
                        {
                            if count < self.code_list.len() {
                                if let StmtElem::UnKnownElem(next_e) = &self.code_list[count + 1] {
                                    if Self::COMMENT_CLOSE.ends_with(next_e.contents)
                                    // "/" == e.content
                                    {
                                        rlist.push(StmtElem::CommentElem(CommentBranch {
                                            contents: group.clone(),
                                            depth: self.depth,
                                            loopdepth: self.loopdepth,
                                        }));
                                        group.clear();
                                        open_status = StringAreaState::Closed;
                                        ignore_flag = true;
                                    } else {
                                        group.push(e.contents);
                                    }
                                } else {
                                    // defer type
                                    return Err(ParserError::UnexpectedType);
                                }
                            } else {
                                return Err(ParserError::CommentBlockNotClosed);
                            }
                        } else {
                            group.push(e.contents);
                        }
                    }
                    StringAreaState::QuotationOpen => {
                        // '"' is opened
                        if Self::DOUBLE_QUOTATION == e.contents {
                            if string_escape_flag {
                                group.push(e.contents);
                                string_escape_flag = false;
                            } else {
                                rlist.push(StmtElem::StringElem(StringBranch {
                                    contents: group.clone(),
                                    depth: self.depth,
                                    loopdepth: self.loopdepth,
                                }));
                                group.clear();
                                open_status = StringAreaState::Closed;
                            }
                        } else if Self::ESCAPECHAR == e.contents {
                            if string_escape_flag {
                                group.push(e.contents);
                                string_escape_flag = false;
                            } else {
                                string_escape_flag = true;
                            }
                        } else {
                            group.push(e.contents);
                        }
                    }
                    StringAreaState::Closed => {
                        // 何も開いていないとき
                        if Self::COMMENT_OPEN.starts_with(e.contents)
                            || Self::COMMENT_START.starts_with(e.contents)
                        {
                            if count < self.code_list.len() {
                                if let StmtElem::UnKnownElem(next_e) = &self.code_list[count + 1] {
                                    if Self::COMMENT_OPEN.ends_with(next_e.contents) {
                                        open_status = StringAreaState::CommentOpen;
                                        ignore_flag = true;
                                    } else if Self::COMMENT_START.ends_with(next_e.contents) {
                                        open_status = StringAreaState::CommentStart;
                                        ignore_flag = true;
                                    } else {
                                        rlist.push(inner.clone())
                                    }
                                } else {
                                    rlist.push(inner.clone());
                                }
                            } else {
                                rlist.push(inner.clone());
                            }
                        } else if Self::DOUBLE_QUOTATION == e.contents {
                            open_status = StringAreaState::QuotationOpen;
                        } else {
                            rlist.push(inner.clone());
                        }
                    }
                }
            } else {
                rlist.push(inner.clone());
            }
        }
        if let StringAreaState::CommentStart = open_status {
            rlist.push(StmtElem::CommentElem(CommentBranch {
                contents: group.clone(),
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        } else if let StringAreaState::CommentOpen = open_status {
            return Err(ParserError::CommentBlockNotClosed);
        } else if let StringAreaState::QuotationOpen = open_status {
            return Err(ParserError::QuotationNotClosed);
        }
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_elements<T>(
        &mut self,
        elemtype: fn(T) -> StmtElem,
        open_char: char,
        close_char: char,
    ) -> Result<(), ParserError>
    where
        T: ASTAreaBranch,
    {
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut group: Vec<StmtElem> = Vec::new();
        let mut depth: isize = 0;

        for inner in &self.code_list {
            if let StmtElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    match depth {
                        0 => { /*pass*/ }
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    match depth {
                        0 => {
                            rlist.push(elemtype(ASTAreaBranch::new(
                                group.clone(),
                                self.depth,
                                self.loopdepth,
                            )));
                            group.clear();
                        }
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                } else {
                    match depth {
                        0 => rlist.push(inner.clone()),
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::BraceNotOpened),
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

    fn semicolon_split(&mut self) -> Result<(), ParserError> {
        // Self::SEMICOLON
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut group: Vec<StmtElem> = Vec::new();
        for inner in &self.code_list {
            match inner {
                StmtElem::UnKnownElem(ukb) => {
                    if ukb.contents == Self::SEMICOLON {
                        // WARN:変則的なトークンであるため注意
                        rlist.push(StmtElem::StmtElem(StmtBranch {
                            code_list: group.clone(),
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
            // WARN:変則的なトークンであるため注意
            rlist.push(StmtElem::StmtElem(StmtBranch {
                code_list: group.clone(),
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        }
        self.code_list = rlist;
        Ok(())
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
