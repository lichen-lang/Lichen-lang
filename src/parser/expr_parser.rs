use crate::abs::ast::*;
use crate::parser::core_parser::*;

use crate::token::func::FuncBranch;
use crate::token::operator::OperatorBranch;
use crate::token::string::StringBranch;
use crate::token::syntax::SyntaxBranch;
use crate::token::syntax_box::SyntaxBoxBranch;
use crate::token::unknown::UnKnownBranch;
use crate::token::word::WordBranch;

use crate::errors::parser_errors::ParserError;

pub struct ExprParser {
    // TODO: 一時的にpublicにしているだけ
    pub code: String,
    pub code_list: Vec<BaseElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ExprParser {
    pub fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: code,
            code_list: Vec::new(),
            depth: depth,
            loopdepth: loopdepth,
        }
    }

    pub fn create_parser_from_vec(
        code_list: Vec<BaseElem>,
        depth: isize,
        loopdepth: isize,
    ) -> Self {
        Self {
            code: String::new(),
            code_list: code_list,
            depth: depth,
            loopdepth: loopdepth,
        }
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
                        }));
                        group.clear();
                    }
                } else if Self::EXCLUDE_WORDS.contains(&e.contents)
                // inner in split
                {
                    if !group.is_empty() {
                        rlist.push(BaseElem::WordElem(WordBranch {
                            contents: group.clone(),
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
                    }));
                    group.clear();
                }
                rlist.push(inner.clone());
            }
        }
        if !group.is_empty() {
            rlist.push(BaseElem::WordElem(WordBranch {
                contents: group.clone(),
            }));
            group.clear();
        }
        self.code_list = rlist;
        return Ok(());
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
                } else {
                    if v.contents == '"'
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
                    } else {
                        if open_flag {
                            if v.contents == '\\' {
                                escape_flag = true;
                            } else {
                                escape_flag = false;
                            }
                            group.push(v.contents);
                        } else {
                            rlist.push(inner.clone());
                        }
                    }
                }
            } else {
                rlist.push(inner.clone());
            }
        }
        if open_flag {
            return Err(ParserError::QuotationNotClosed);
        }
        self.code_list = rlist;
        return Ok(());
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
        return Ok(());
    }

    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        // --- macro ---
        macro_rules! err_proc {
            ($a:expr) => {
                if let Err(e) = $a {
                    return Err(e);
                }
            };
        }
        err_proc!(self.grouping_quotation());
        err_proc!(self.grouping_elements(
            BaseElem::BlockElem,
            Self::BLOCK_BRACE_OPEN,  // {
            Self::BLOCK_BRACE_CLOSE, // }
        ));
        err_proc!(self.grouping_elements(
            BaseElem::ListBlockElem,
            Self::BLOCK_LIST_OPEN,  // [
            Self::BLOCK_LIST_CLOSE, // ]
        ));
        err_proc!(self.grouping_elements(
            BaseElem::ParenBlockElem,
            Self::BLOCK_PAREN_OPEN,  // (
            Self::BLOCK_PAREN_CLOSE, // )
        ));
        err_proc!(self.grouoping_operator2());
        err_proc!(self.grouping_words());
        return Ok(());
    }

    fn grouoping_operator2(&mut self) -> Result<(), ParserError> {
        for ope in Self::LENGTH_ORDER_OPE_LIST {
            if let Err(e) = self.grouoping_operator_unit2(ope.opestr.to_string()) {
                return Err(e);
            }
        }
        return Ok(());
    }

    fn grouoping_operator_unit2(&mut self, ope: String) -> Result<(), ParserError> {
        let mut group: String = String::new();
        let mut rlist: Vec<BaseElem> = Vec::new();

        let ope_size = ope.len();
        for inner in &self.code_list {
            if let BaseElem::UnKnownElem(e) = inner {
                // 未解決の場合
                group.push(e.contents);
                if group.len() < ope_size {
                    continue;
                } else if ope_size == group.len() {
                    if group == ope {
                        rlist.push(BaseElem::OpeElem(OperatorBranch {
                            ope: group.clone(),
                            depth: self.depth,
                        }))
                    } else {
                        // rlist += group
                        let grouup_tmp: Vec<BaseElem> = group
                            .chars()
                            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                            .collect();
                        rlist.extend(grouup_tmp);
                    }
                } else {
                    // ope_size < group.len()
                    // rlist += group
                    let grouup_tmp: Vec<BaseElem> = group
                        .chars()
                        .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                        .collect();
                    rlist.extend(grouup_tmp);
                }
                group.clear();
            } else {
                // 既にtokenが割り当てられているとき
                if group.len() < ope_size {
                    // rlist += group
                    let grouup_tmp: Vec<BaseElem> = group
                        .chars()
                        .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                        .collect();
                    rlist.extend(grouup_tmp);
                } else if ope_size == group.len() {
                    if group == ope {
                        rlist.push(BaseElem::OpeElem(OperatorBranch {
                            ope: group.clone(),
                            depth: self.depth,
                        }))
                    } else {
                        // rlist += group
                        let grouup_tmp: Vec<BaseElem> = group
                            .chars()
                            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                            .collect();
                        rlist.extend(grouup_tmp);
                    }
                } else {
                    // rlist += group
                    let grouup_tmp: Vec<BaseElem> = group
                        .chars()
                        .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                        .collect();
                    rlist.extend(grouup_tmp);
                }
                group.clear();
                rlist.push(inner.clone());
            }
        } //end of "for inner in codelist"
        self.code_list = rlist;
        return Ok(());
    }

    fn grouping_syntaxbox(&mut self) -> Result<(), ParserError> {
        let mut flag = false;
        let mut name: String = String::new();
        let mut group: Vec<SyntaxBranch> = Vec::new();
        let mut rlist: Vec<BaseElem> = Vec::new();

        for inner in &self.code_list {
            if let BaseElem::SyntaxElem(ref e) = inner {
                if Self::SYNTAX_WORDS_HEADS.contains(&e.name.as_str()) {
                    flag = true;
                    name = e.name.clone();
                    group.push(e.clone());
                } else if e.name == "elif" {
                    if flag {
                        group.push(e.clone());
                    } else {
                        return Err(ParserError::GroupingSyntaxBoxError);
                        // TODO:
                    }
                } else if e.name == "else" {
                    if flag {
                        group.push(e.clone());
                        rlist.push(BaseElem::SyntaxBoxElem(SyntaxBoxBranch {
                            name: name.clone(),
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                        name = String::from("");
                        flag = false;
                    } else {
                        return Err(ParserError::GroupingSyntaxBoxError);
                        // TODO:
                    }
                } else {
                    rlist.push(inner.clone());
                }
            } else {
                if flag {
                    if !group.is_empty() {
                        rlist.push(BaseElem::SyntaxBoxElem(SyntaxBoxBranch {
                            name: name.clone(),
                            contents: group.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                        group.clear();
                        name = String::from("");
                    } else {
                        //pass
                    }
                    flag = false;
                } else {
                    //pass
                }
                rlist.push(inner.clone());
            }
        }
        if !group.is_empty() {
            rlist.push(BaseElem::SyntaxBoxElem(SyntaxBoxBranch {
                name: name.clone(),
                contents: group.clone(),
                depth: self.depth,
                loopdepth: self.loopdepth,
            }));
        }
        self.code_list = rlist;
        return Ok(());
    }

    ///
    /// TODO: Word以外について`()`が付与され呼ばれたときに
    /// 関数として認識できるようにする必要がある
    /// 例えば以下のような場合について
    /// ```lichen
    /// funcA()() // 関数を返却するような関数
    /// a[]()     // 関数を保持しているリスト
    /// ```
    fn grouping_functioncall<T>(&mut self) -> Result<(), ParserError> {
        let mut flag: bool = false;
        let mut name_tmp: Option<BaseElem> = None;
        let mut rlist: Vec<BaseElem> = Vec::new();

        for inner in &self.code_list {
            if let BaseElem::WordElem(ref wb) = inner {
                // Case WordElem
                if flag {
                    if let Some(e) = name_tmp {
                        rlist.push(e);
                    }
                }
                name_tmp = Some(inner.clone());
                flag = true;
            } else if let BaseElem::FuncElem(ref fb) = inner {
                // Case FuncElem
                if flag {
                    if let Some(e) = name_tmp {
                        rlist.push(e);
                    }
                }
                name_tmp = Some(inner.clone());
                flag = true;
            } else if let BaseElem::ParenBlockElem(ref pbb) = inner {
                // Case ParenBlockElem
                if flag {
                    if let Some(ref base_e) = name_tmp {
                        if let BaseElem::WordElem(ref wb) = base_e {
                            if Self::CONTROL_STATEMENT.contains(&(&wb.contents as &str)) {
                                rlist.push(BaseElem::FuncElem(FuncBranch {
                                    name: Box::new(base_e.clone()),
                                    contents: pbb.clone(),
                                    depth: self.depth,
                                    loopdepth: self.loopdepth,
                                }));
                                name_tmp = None;
                                flag = false;
                            } else {
                                // name tmp is not none
                                rlist.push(base_e.clone()); // contents of name_tmp -> base_e
                                rlist.push(inner.clone());
                                name_tmp = None;
                            }
                        } else if let BaseElem::FuncElem(_) = base_e {
                            rlist.push(BaseElem::FuncElem(FuncBranch {
                                name: Box::new(base_e.clone()),
                                contents: pbb.clone(),
                                depth: self.depth,
                                loopdepth: self.loopdepth,
                            }));
                            name_tmp = None;
                            flag = false;
                        } else {
                            // name tmp is not none
                            rlist.push(base_e.clone()); // contents of name_tmp -> base_e
                            rlist.push(inner.clone());
                            name_tmp = None;
                        }
                    } else {
                        //name tmp is none
                        rlist.push(inner.clone());
                        flag = false;
                        name_tmp = None;
                    }
                }
            } else {
                // pass
            }
        }
        if flag {
            if let Some(e) = name_tmp {
                rlist.push(e);
            }
        }
        self.code_list = rlist;
        return Ok(());
    }

    fn resolve_operation(&mut self) {}
}

impl Parser<'_> for ExprParser {
    fn resolve(&mut self) -> Result<(), ParserError> {
        self.code_list = self.code2_vec_pre_proc_func(&self.code);
        if let Err(e) = self.code2vec() {
            return Err(e);
        } else {
            for i in &mut self.code_list {
                if let Err(e) = i.resolve_self() {
                    return Err(e);
                }
            }
            return Ok(());
        }
    }
}
