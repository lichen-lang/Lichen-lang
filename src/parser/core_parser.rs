// use crate::parser::token::*;
use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::token::unknown::UnKnownBranch;

pub enum Prio {
    Left,
    Right,
    Prefix,
}

pub struct Ope<'a> {
    pub opestr: &'a str,
    pub priority_direction: Prio,
    pub priority: i32,
}

/// macro for Parser trait
macro_rules! def_ope {
    ($name:ident,$string:expr,$prio_direction:path,$prio:expr) => {
        const $name: &'a Ope<'a> = &Ope {
            opestr: $string,
            priority_direction: $prio_direction,
            priority: $prio,
        };
    };
}

/// # Parser trait
/// パーサのコア実装
pub trait Parser<'a> {
    // operators
    // - left priority
    //   - priority -3
    def_ope!(OR, "||", Prio::Left, -3);
    //   - priority -2
    def_ope!(AND, "&&", Prio::Left, -2);
    //   - priority 0
    def_ope!(EQ, "==", Prio::Left, 0);
    def_ope!(NE, "!=", Prio::Left, 0);
    def_ope!(LT, "<", Prio::Left, 0);
    def_ope!(LE, "<=", Prio::Left, 0);
    def_ope!(GT, ">", Prio::Left, 0);
    def_ope!(GE, ">=", Prio::Left, 0);
    //   - priority 1
    def_ope!(ADD, "+", Prio::Left, 1);
    def_ope!(SUB, "-", Prio::Left, 1);
    def_ope!(MUL, "*", Prio::Left, 2);
    def_ope!(DIV, "/", Prio::Left, 2);
    def_ope!(MOD, "%", Prio::Left, 2);
    def_ope!(DOT, "@", Prio::Left, 2);

    // - right priority
    //   - priority -4
    def_ope!(ASSIGNMENT, "=", Prio::Right, -4);
    def_ope!(ADDEQ, "+=", Prio::Right, -4);
    def_ope!(SUBEQ, "-=", Prio::Right, -4);
    def_ope!(MULEQ, "*=", Prio::Right, -4);
    def_ope!(DIVEQ, "/=", Prio::Right, -4);
    def_ope!(MODEQ, "%=", Prio::Right, -4);
    //   - priority -3
    def_ope!(POW, "**", Prio::Right, 3);

    // - prefix priority
    //   - priority -1
    def_ope!(NOT, "!", Prio::Prefix, -1);

    /// 演算子を文字列として長いものからの順番で並べたもの
    const LENGTH_ORDER_OPE_LIST: [&'a Ope<'a>; 22] = [
        // length 2
        Self::OR,    // ||
        Self::AND,   // &&
        Self::EQ,    // ==
        Self::NE,    // !=
        Self::LE,    // <=
        Self::GE,    // >=
        Self::ADDEQ, // +=
        Self::SUBEQ, // -=
        Self::MULEQ, // *=
        Self::DIVEQ, // /=
        Self::MODEQ, // %=
        Self::POW,   // **
        // length 1
        Self::LT,         // <
        Self::GT,         // >
        Self::ADD,        // +
        Self::SUB,        // -
        Self::MUL,        // *
        Self::DIV,        // /
        Self::MOD,        // %
        Self::DOT,        // @
        Self::ASSIGNMENT, // =
        Self::NOT,        // !
    ];

    const SEMICOLON: char = ';';
    const COMMA: char = ',';
    const SPLIT_CHAR: [char; 3] = [' ', '\t', '\n'];
    const EXCLUDE_WORDS: [char; 3] = [Self::SEMICOLON, ':', Self::COMMA];

    const SYNTAX_IF: &'a str = "if";
    const SYNTAX_ELIF: &'a str = "elif";
    const SYNTAX_ELSE: &'a str = "else";
    const SYNTAX_LOOP: &'a str = "loop";
    const SYNTAX_FOR: &'a str = "for";
    const SYNTAX_WHILE: &'a str = "while";
    const SYNTAX_MATCH: &'a str = "match";

    const SYNTAX_WORDS: [&'a str; 7] = [
        Self::SYNTAX_IF,    // if   (){}
        Self::SYNTAX_ELIF,  // elif (){}
        Self::SYNTAX_ELSE,  // else {}
        Self::SYNTAX_LOOP,  // loop {}
        Self::SYNTAX_FOR,   // for  (){}
        Self::SYNTAX_WHILE, // while(){}
        Self::SYNTAX_MATCH, // match(){}
    ];
    const SYNTAX_WORDS_HEADS: [&'a str; 4] = [
        Self::SYNTAX_IF,    // if   (){} ...
        Self::SYNTAX_LOOP,  // loop (){} ...
        Self::SYNTAX_FOR,   // for  (){} ...
        Self::SYNTAX_WHILE, // while(){} ...
    ];
    const ESCAPECHAR: char = '\\';
    const FUNCTION: &'a str = "fn";
    const STRUCTURE: &'a str = "struct";
    const ENUMERATION: &'a str = "enum";
    const DOUBLE_QUOTATION: char = '"';
    const SINGLE_QUOTATION: char = '\'';

    const CONTROL_RETURN: &'a str = "return";
    const CONTROL_BREAK: &'a str = "break";
    const CONTROL_CONTINUE: &'a str = "continue";
    const CONTROL_ASSERT: &'a str = "assert";

    const CONTROL_STATEMENT: [&'a str; 4] = [
        Self::CONTROL_RETURN,   // return
        Self::CONTROL_BREAK,    // break
        Self::CONTROL_CONTINUE, // continue
        Self::CONTROL_ASSERT,   // assert
    ];

    const KEYWORDS: [&'a str; 14] = [
        // Syntax
        Self::SYNTAX_IF,    // if
        Self::SYNTAX_ELIF,  // elif
        Self::SYNTAX_ELSE,  // else
        Self::SYNTAX_LOOP,  // loop
        Self::SYNTAX_FOR,   // match
        Self::SYNTAX_WHILE, // while
        Self::SYNTAX_MATCH, // match
        // keyword
        Self::FUNCTION,    // fn
        Self::STRUCTURE,   // struct
        Self::ENUMERATION, // enum
        // control
        Self::CONTROL_RETURN,   // return
        Self::CONTROL_BREAK,    // break
        Self::CONTROL_CONTINUE, // control
        Self::CONTROL_ASSERT,   // assert
    ];

    const BLOCK_BRACE_OPEN: char = '{';
    const BLOCK_BRACE_CLOSE: char = '}';
    const BLOCK_PAREN_OPEN: char = '(';
    const BLOCK_PAREN_CLOSE: char = ')';
    const BLOCK_LIST_OPEN: char = '[';
    const BLOCK_LIST_CLOSE: char = ']';

    // fn code2vec(&self, code: &Vec<BaseElem>) -> Result<Vec<BaseElem>, &str>;

    fn new(code: String, depth: isize, loopdepth: isize) -> Self;
    fn resolve(&mut self) -> Result<(), ParserError>;
    fn create_parser_from_vec(code_list: Vec<ExprElem>, depth: isize, loopdepth: isize) -> Self;

    fn code2_vec_pre_proc_func(code: &str) -> Vec<ExprElem> {
        return code
            .chars()
            .map(|c| ExprElem::UnKnownElem(UnKnownBranch { contents: c }))
            .collect();
    }
}
