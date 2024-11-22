use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::*;

use crate::token::operator::OperatorBranch;
use crate::token::stmt::expr::ExprBranch;
use crate::token::stmt::stmt::StmtBranch;
use crate::token::string::StringBranch;
use crate::token::word::WordBranch;
use crate::token::comment::CommentBranch;

/// # StmtParser
pub struct StmtParser {
    pub code: String,
    pub code_list: Vec<StmtElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

enum StringAreaState {
    CommentOpen,  // /*
    CommentStart, // //
    QuotationOpen,
    Closed,
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
        self.split_semicolon()?;
        Ok(())
    }

        fn grouping_string(&mut self) -> Result<(), ParserError> {
        // now this function can group all string in  the program
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

    /// function for splitting semicolon
    ///
    /// ```text
    /// let a = 1; // <- stmt
    /// let b = 2; // <- stmt
    /// let c = 3; // <- stmt
    /// return a; // <- stmt
    /// ```
    pub fn split_semicolon(&mut self) -> Result<(), ParserError> {
        let mut rlist: Vec<StmtElem> = Vec::new();
        let mut group: Vec<StmtElem> = Vec::new();

        for inner in &self.code_list {
            match &inner {
                StmtElem::UnKnownElem(unb) => {
                    if unb.contents == Self::SEMICOLON {
                        if !group.is_empty() {
                            if let StmtElem::WordElem(word_b) = &group[0]{
                                if Self::CONTROL_STATEMENT.contains(&word_b.contents.as_str()) {
                                    // return 等の
                                    // 予約語だった場合
                                    rlist.push(StmtElem::Special(StmtBranch { 
                                        head: word_b.contents.clone(),
                                        code_list: Self::stmt2expr(&group[1..])?,
                                        depth: self.depth,
                                        loopdepth: self.loopdepth
                                    }));
                                }else{
                                    // 普通の変数のwordだった場合
                                    rlist.push(StmtElem::ExprElem(ExprBranch {
                                        code_list: Self::stmt2expr(&group)?,
                                        depth: self.depth,
                                        loopdepth: self.loopdepth,
                                    }));
                                }
                            } else  {
                                // 最初の要素がwordではなかった場合
                                rlist.push(StmtElem::ExprElem(ExprBranch {
                                    code_list: Self::stmt2expr(&group)?,
                                    depth: self.depth,
                                    loopdepth: self.loopdepth,
                                }));
                            }
                        } else {
                            // group が空だった場合
                        }
                        group.clear();
                    } else {
                        // セミコロン以外でまだ決定していないchar
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
                code_list: Self::stmt2expr(&group)?,
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        }
        self.code_list = rlist;
        Ok(())
    }

    /// function for converting `stmt` to `expr`
    fn stmt2expr(i: &[StmtElem]) -> Result<Vec<ExprElem>, ParserError> {
        let mut rlist: Vec<ExprElem> = Vec::new();
        for inner in i.iter() {
            rlist.push(match inner {
                StmtElem::StringElem(a) => ExprElem::StringElem(a.clone()),
                StmtElem::CommentElem(a) => ExprElem::CommentElem(a.clone()),
                StmtElem::BlockElem(a) => ExprElem::BlockElem(a.clone()),
                StmtElem::ListBlockElem(a) => ExprElem::ListBlockElem(a.clone()),
                StmtElem::ParenBlockElem(a) => ExprElem::ParenBlockElem(a.clone()),
                StmtElem::OpeElem(a) => ExprElem::OpeElem(a.clone()),
                StmtElem::WordElem(a) => ExprElem::WordElem(a.clone()),
                StmtElem::UnKnownElem(a) => ExprElem::UnKnownElem(a.clone()),
                _ => {
                    return Err(ParserError::UnableToConvertType)
                },
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
