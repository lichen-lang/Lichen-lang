use std::cmp::Ordering;

use crate::abs::ast::*;
use crate::parser::core_parser::*;

use crate::errors::parser_errors::ParserError;
use crate::token::func::FuncBranch;
use crate::token::list::ListBranch;
use crate::token::operator::OperatorBranch;
use crate::token::paren_block::ParenBlockBranch;
use crate::token::string::StringBranch;
use crate::token::syntax::SyntaxBranch;
use crate::token::syntax_box::SyntaxBoxBranch;
use crate::token::unknown::UnKnownBranch;
use crate::token::word::WordBranch;

pub struct ExprParser {
    pub code: String,
    pub code_list: Vec<BaseElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

macro_rules! subscriptable_pat {
    (
        $self:ident,
        word: $w:path|
        callable: $($a:path),+|
        block: $b:path|
        call: $e: path|
        callbranch: $br:ident
    ) => {{
    let mut name_tmp: Option<BaseElem> = None;
    let mut rlist: Vec<BaseElem> = Vec::new();
        for inner in &$self.code_list {
            match inner {
                $(
                        $a(_v) => {
                        if let Some(s) = name_tmp {
                            rlist.push(s);
                        }
                        name_tmp = Some(inner.clone());
                        }
                ) +
                $b(v) => {
                    if let Some($w(ref wd)) = name_tmp {
                        if !Self::CONTROL_STATEMENT.contains(&wd.contents.as_str()) {
                            rlist.push(
                            $br::create_elem(
                                Box::new($w(wd.clone())),
                                v.clone(),
                                $self.depth,
                                $self.loopdepth,
                            )
                            );
                        } else {
                            // 1
                            if let Some(ref s) = name_tmp {
                                rlist.push(s.clone());
                            }
                            rlist.push(inner.clone());
                        }
                    }
                    $(
                        else if let Some($a(ref fb)) = name_tmp {
                        rlist.push(
                            $br::create_elem(
                                Box::new($a(fb.clone())),
                                v.clone(),
                                $self.depth,
                                $self.loopdepth,
                            )
                        );
                    }
                    )+
                    else {
                        // 1
                        if let Some(ref s) = name_tmp {
                            rlist.push(s.clone());
                        }
                        rlist.push(inner.clone());
                    }
                    name_tmp = None;
                }
                _ => {
                    if let Some(ref s) = name_tmp {
                        rlist.push(s.clone());
                        rlist.push(inner.clone());
                        name_tmp = None;
                    } else {
                        rlist.push(inner.clone());
                    }
                }
            }
        }
        if let Some(ref s) = name_tmp {
            rlist.push(s.clone());
        }
        $self.code_list = rlist;
        Ok(())
    }}
}

impl ExprParser {
    fn grouping_words(&mut self) -> Result<(), ParserError> {
        // macro
        macro_rules! add_rlist {
            ($rlist:expr,$group:expr) => {
                if let Ok(_) = self.find_ope_priority(&$group) {
                    $rlist.push(BaseElem::OpeElem(OperatorBranch {
                        ope: $group.clone(),
                        depth: self.depth,
                    }))
                } else {
                    $rlist.push(BaseElem::WordElem(WordBranch {
                        contents: $group.clone(),
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    }));
                }
            };
        }
        let mut rlist: Vec<BaseElem> = Vec::new();
        let mut group: String = String::new();
        let ope_str = Self::LENGTH_ORDER_OPE_LIST.map(|a| a.opestr).join("");

        for inner in &self.code_list {
            if let BaseElem::UnKnownElem(ref e) = inner {
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

    pub fn code2vec(&mut self) -> Result<(), ParserError> {
        // --- macro ---
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
        // end of grouping_elements
        self.grouping_words()?;
        while self.contain_callable() {
            self.grouping_functioncall2()?;
        }
        while self.contain_subscriptable() {
            println!("hellooooooooooooooooooooooooooooooooooo");
            self.grouping_subscription2()?;
        }
        self.grouoping_operator()?;
        self.resolve_operation()?;
        Ok(())
    }

    fn grouoping_operator(&mut self) -> Result<(), ParserError> {
        for ope in Self::LENGTH_ORDER_OPE_LIST {
            self.grouoping_operator_unit(ope.opestr.to_string())?;
        }
        Ok(())
    }

    fn grouoping_operator_unit(&mut self, ope: String) -> Result<(), ParserError> {
        let mut group: String = String::new();
        let mut rlist: Vec<BaseElem> = Vec::new();

        let ope_size: usize = ope.len();
        for inner in &self.code_list {
            if let BaseElem::UnKnownElem(e) = inner {
                // 未解決の場合
                group.push(e.contents);
                match group.len().cmp(&ope_size) {
                    Ordering::Less => {}
                    Ordering::Equal => {
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
                        group.clear();
                    }
                    Ordering::Greater => {
                        // ope_size < group.len()
                        // rlist += group
                        let grouup_tmp: Vec<BaseElem> = group
                            .chars()
                            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                            .collect();
                        rlist.extend(grouup_tmp);
                        group.clear();
                    }
                }
            } else {
                // 既にtokenが割り当てられているとき
                match group.len().cmp(&ope_size) {
                    Ordering::Less => {
                        let grouup_tmp: Vec<BaseElem> = group
                            .chars()
                            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                            .collect();
                        rlist.extend(grouup_tmp);
                    }
                    Ordering::Equal => {
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
                    }
                    Ordering::Greater => {
                        // rlist += group
                        let grouup_tmp: Vec<BaseElem> = group
                            .chars()
                            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
                            .collect();
                        rlist.extend(grouup_tmp);
                    }
                }
                group.clear();
                rlist.push(inner.clone());
            }
        } //end of "for inner in codelist"
        self.code_list = rlist;
        Ok(())
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
                    name.clone_from(&e.name);
                    group.push(e.clone());
                } else if e.name == Self::SYNTAX_ELIF {
                    if flag {
                        group.push(e.clone());
                    } else {
                        return Err(ParserError::GroupingSyntaxBoxError);
                        // TODO:
                    }
                } else if e.name == Self::SYNTAX_ELSE {
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
        Ok(())
    }

    fn contain_callable(&self) -> bool {
        let mut flag = false;
        let mut name_tmp: Option<&BaseElem> = None;

        for inner in &self.code_list {
            match inner {
                BaseElem::WordElem(_) | BaseElem::FuncElem(_) => {
                    name_tmp = Some(inner);
                    flag = true;
                }
                BaseElem::ParenBlockElem(_) => {
                    if let Some(BaseElem::WordElem(v)) = name_tmp {
                        if flag && !Self::CONTROL_STATEMENT.contains(&v.contents.as_str()) {
                            return true;
                        }
                    } else if let Some(BaseElem::FuncElem(_v)) = name_tmp {
                        return true;
                    } else {
                        name_tmp = None;
                        flag = false;
                    }
                }
                _ => {
                    if flag {
                        flag = false;
                        name_tmp = None;
                    } else {
                        //pass
                    }
                }
            }
        }
        false
    }

    //
    // TODO: Word以外について`()`が付与され呼ばれたときに
    // 関数として認識できるようにする必要がある
    // 例えば以下のような場合について
    // ```lichen
    // funcA()() // 関数を返却するような関数
    // a[]()     // 関数を保持しているリスト
    // ```
    fn grouping_functioncall(&mut self) -> Result<(), ParserError> {
        let mut name_tmp: Option<BaseElem> = None;
        let mut rlist: Vec<BaseElem> = Vec::new();

        for inner in &self.code_list {
            match inner {
                BaseElem::WordElem(_v) => {
                    if let Some(s) = name_tmp {
                        rlist.push(s);
                    }
                    name_tmp = Some(inner.clone());
                }
                BaseElem::FuncElem(_v) => {
                    if let Some(s) = name_tmp {
                        rlist.push(s);
                    }
                    name_tmp = Some(inner.clone());
                }
                BaseElem::ParenBlockElem(v) => {
                    if let Some(BaseElem::WordElem(ref wd)) = name_tmp {
                        if !Self::CONTROL_STATEMENT.contains(&wd.contents.as_str()) {
                            rlist.push(BaseElem::FuncElem(FuncBranch {
                                name: Box::new(BaseElem::WordElem(wd.clone())),
                                contents: v.clone(),
                                out_code_list: Vec::new(),
                                depth: self.depth,
                                loopdepth: self.loopdepth,
                            }));
                        } else {
                            // 1
                            if let Some(ref s) = name_tmp {
                                rlist.push(s.clone());
                            }
                            rlist.push(inner.clone());
                        }
                    } else if let Some(BaseElem::FuncElem(ref fb)) = name_tmp {
                        rlist.push(BaseElem::FuncElem(FuncBranch {
                            name: Box::new(BaseElem::FuncElem(fb.clone())),
                            contents: v.clone(),
                            out_code_list: Vec::new(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                    } else {
                        // 1
                        if let Some(ref s) = name_tmp {
                            rlist.push(s.clone());
                        }
                        rlist.push(inner.clone());
                    }
                    name_tmp = None;
                }
                _ => {
                    if let Some(ref s) = name_tmp {
                        rlist.push(s.clone());
                        rlist.push(inner.clone());
                        name_tmp = None;
                    } else {
                        rlist.push(inner.clone());
                    }
                }
            }
        }
        if let Some(ref s) = name_tmp {
            rlist.push(s.clone());
        }
        self.code_list = rlist;
        Ok(())
    }

    fn contain_subscriptable(&self) -> bool {
        let mut flag = false;
        let mut name_tmp: Option<&BaseElem> = None;

        for inner in &self.code_list {
            match inner {
                BaseElem::WordElem(_) | BaseElem::FuncElem(_) => {
                    name_tmp = Some(inner);
                    flag = true;
                }
                BaseElem::ListBlockElem(_) => {
                    if let Some(BaseElem::WordElem(v)) = name_tmp {
                        if flag && !Self::CONTROL_STATEMENT.contains(&v.contents.as_str()) {
                            return true;
                        }
                    } else if let Some(BaseElem::FuncElem(_v)) = name_tmp {
                        return true;
                    } else {
                        name_tmp = None;
                        flag = false;
                    }
                }
                _ => {
                    if flag {
                        flag = false;
                        name_tmp = None;
                    } else {
                        //pass
                    }
                }
            }
        }
        false
    }

    fn grouping_subscription(&mut self) -> Result<(), ParserError> {
        let mut name_tmp: Option<BaseElem> = None;
        let mut rlist: Vec<BaseElem> = Vec::new();

        for inner in &self.code_list {
            match inner {
                BaseElem::WordElem(_v) => {
                    if let Some(s) = name_tmp {
                        rlist.push(s);
                    }
                    name_tmp = Some(inner.clone());
                }
                BaseElem::FuncElem(_v) => {
                    if let Some(s) = name_tmp {
                        rlist.push(s);
                    }
                    name_tmp = Some(inner.clone());
                }
                BaseElem::ListElem(_v) => {
                    if let Some(s) = name_tmp {
                        rlist.push(s);
                    }
                    name_tmp = Some(inner.clone());
                }
                BaseElem::ListBlockElem(v) => {
                    if let Some(BaseElem::WordElem(ref wd)) = name_tmp {
                        if !Self::CONTROL_STATEMENT.contains(&wd.contents.as_str()) {
                            rlist.push(BaseElem::ListElem(ListBranch {
                                name: Box::new(BaseElem::WordElem(wd.clone())),
                                contents: v.clone(),
                                depth: self.depth,
                                loopdepth: self.loopdepth,
                            }));
                        } else {
                            // 1
                            if let Some(ref s) = name_tmp {
                                rlist.push(s.clone());
                            }
                            rlist.push(inner.clone());
                        }
                    } else if let Some(BaseElem::FuncElem(ref fb)) = name_tmp {
                        rlist.push(BaseElem::ListElem(ListBranch {
                            name: Box::new(BaseElem::FuncElem(fb.clone())),
                            contents: v.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                    } else if let Some(BaseElem::ListElem(ref lb)) = name_tmp {
                        rlist.push(BaseElem::ListElem(ListBranch {
                            name: Box::new(BaseElem::ListElem(lb.clone())),
                            contents: v.clone(),
                            depth: self.depth,
                            loopdepth: self.loopdepth,
                        }));
                    } else {
                        // 1
                        if let Some(ref s) = name_tmp {
                            rlist.push(s.clone());
                        }
                        rlist.push(inner.clone());
                    }
                    name_tmp = None;
                }
                _ => {
                    if let Some(ref s) = name_tmp {
                        rlist.push(s.clone());
                        rlist.push(inner.clone());
                        name_tmp = None;
                    } else {
                        rlist.push(inner.clone());
                    }
                }
            }
        }
        if let Some(ref s) = name_tmp {
            rlist.push(s.clone());
        }
        self.code_list = rlist;
        Ok(())
    }

    fn grouping_subscription2(&mut self) -> Result<(), ParserError> {
        // 各々のセクションが表しているもの
        // word: BaseElem::WordElem
        // callable : 関数またはリストとして、呼べるもの。この時点では構文解析なので、型的に実際に呼べるかは関係ない
        // block : ()[]などのブロック
        // call : 最終的なASTを作るのに必要なインスタンス
        // callbranch : 上に同じ
        subscriptable_pat!(
            self,
            word: BaseElem::WordElem |
            callable: BaseElem::WordElem,BaseElem::ListElem, BaseElem::FuncElem |
            block: BaseElem::ListBlockElem|
            call: BaseElem::ListElem|
            callbranch: ListBranch
        )
    }

    fn grouping_functioncall2(&mut self) -> Result<(), ParserError> {
        subscriptable_pat!(
            self,
            word: BaseElem::WordElem |
            callable: BaseElem::WordElem,BaseElem::ListElem, BaseElem::FuncElem |
            block: BaseElem::ParenBlockElem|
            call: BaseElem::FuncElem|
            callbranch: FuncBranch
        )
    }

    fn find_ope_priority(&self, ope: &str) -> Result<&Ope, ()> {
        for i in Self::LENGTH_ORDER_OPE_LIST {
            if i.opestr == ope {
                return Ok(i);
            }
        }
        Err(())
    }

    fn find_min_priority_index(&self) -> Result<Option<usize>, ParserError> {
        let mut priority_tmp: i32 = i32::MAX;
        let mut index_tmp = None;
        for (index, inner) in self.code_list.iter().enumerate() {
            if let BaseElem::OpeElem(ope) = inner {
                let ope_contents = &ope.ope;
                if let Ok(ope_info) = self.find_ope_priority(ope_contents) {
                    if index < 1
                    // if index == 0:
                    {
                        index_tmp = Some(index);
                        priority_tmp = 4; // unsafe
                    } else if let BaseElem::OpeElem(_) = &self.code_list[index - 1] {
                        continue;
                    } else if ope_info.priority < priority_tmp {
                        index_tmp = Some(index);
                        priority_tmp = ope_info.priority;
                    } else if ope_info.priority == priority_tmp {
                        match ope_info.priority_direction {
                            Prio::Left => {
                                index_tmp = Some(index);
                                priority_tmp = ope_info.priority;
                            }
                            Prio::Right => {}
                            Prio::Prefix => {}
                        }
                    } // else pass
                } else {
                    // error case
                    return Err(ParserError::OperationError);
                }
            } else {
                continue;
            }
        }
        Ok(index_tmp)
    }

    fn resolve_operation(&mut self) -> Result<(), ParserError> {
        let operation_index = self.find_min_priority_index();
        match operation_index {
            Ok(v) => {
                if let Some(s) = v {
                    let arg1 = &self.code_list[..s];
                    let name = &self.code_list[s];
                    let arg2 = &self.code_list[s + 1..];
                    self.code_list = vec![BaseElem::FuncElem(FuncBranch {
                        name: Box::new(name.clone()),
                        contents: ParenBlockBranch {
                            contents: None,
                            depth: 0,
                            loopdepth: 0,
                        },
                        out_code_list: vec![arg1.to_vec(), arg2.to_vec()],
                        depth: self.depth,
                        loopdepth: self.loopdepth,
                    })];
                    Ok(())
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl Parser<'_> for ExprParser {
    fn create_parser_from_vec(code_list: Vec<BaseElem>, depth: isize, loopdepth: isize) -> Self {
        Self {
            code: String::new(),
            code_list,
            depth,
            loopdepth,
        }
    }

    fn new(code: String, depth: isize, loopdepth: isize) -> Self {
        Self {
            code,
            code_list: Vec::new(),
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
