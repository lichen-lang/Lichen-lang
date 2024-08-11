// use crate::parser::token::*;
use crate::abs::ast::*;
use crate::errors::parser_errors::ParserError;
use crate::token::unknown::UnKnownBranch;

/// # Parser trait
/// パーサのコア実装
pub trait Parser<'a> {
    // operators
    const OR: &'a str = "||";
    const AND: &'a str = "&&";
    const EQ: &'a str = "==";
    const NE: &'a str = "!=";
    const LT: &'a str = "<";
    const LE: &'a str = "<=";
    const GT: &'a str = ">";
    const GE: &'a str = ">=";
    const ADD: &'a str = "+";
    const SUB: &'a str = "-";
    const MUL: &'a str = "*";
    const DIV: &'a str = "/";
    const MOD: &'a str = "%";
    const DOT: &'a str = "@";

    const ASSIGNMENT: &'a str = "=";
    const ADDEQ: &'a str = "+=";
    const SUBEQ: &'a str = "-=";
    const MULEQ: &'a str = "*=";
    const DIVEQ: &'a str = "/=";
    const MODEQ: &'a str = "%=";

    const POW: &'a str = "**";
    const NOT: &'a str = "!";

    const LEFT_PRIORITY_LIST: [(&'a str, isize); 14] = [
        (Self::OR, -3),  // ||
        (Self::AND, -2), // &&
        // PRIORITY 0
        (Self::EQ, 0), // ==
        (Self::NE, 0), // !=
        (Self::LT, 0), // <
        (Self::LE, 0), // <=
        (Self::GT, 0), // >
        (Self::GE, 0), // >=
        // PRIORITY 1
        (Self::ADD, 1), // +
        (Self::SUB, 1), // -
        // PRIORITY 2
        (Self::MUL, 2), // *
        (Self::DIV, 2), // /
        (Self::MOD, 2), // %
        (Self::DOT, 2), // @
    ];
    const RIGHT_PRIORITY_LIST: [(&'a str, isize); 7] = [
        // PRIORITY -4
        (Self::ASSIGNMENT, -4), // =
        (Self::ADDEQ, -4),      // +=
        (Self::SUBEQ, -4),      // -=
        (Self::MULEQ, -4),      // *=
        (Self::DIVEQ, -4),      // /=
        (Self::MODEQ, -4),      // %=
        (Self::POW, 3),         // **
    ];
    const PREFIX_PRIORITY_LIST: [(&'a str, isize); 1] = [
        // PRIORITY -1
        (Self::NOT, -1), // !
    ];

    /// 演算子を文字列として長いものからの順番で並べたもの
    const LENGTH_ORDER_OPE_LIST: [&'a str; 22] = [
        Self::OR,         // ||
        Self::AND,        // &&
        Self::EQ,         // ==
        Self::NE,         // !=
        Self::LE,         // <=
        Self::GE,         // >=
        Self::ADDEQ,      // +=
        Self::SUBEQ,      // -=
        Self::MULEQ,      // *=
        Self::DIVEQ,      // /=
        Self::MODEQ,      // %=
        Self::POW,        // **
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

    const SPLIT_CHAR: [char; 3] = [' ', '\t', '\n'];
    const EXCLUDE_WORDS: [char; 3] = [';', ':', ','];

    const SYNTAX_IF: &'a str = "if";
    const SYNTAX_ELIF: &'a str = "elif";
    const SYNTAX_ELSE: &'a str = "else";
    const SYNTAX_LOOP: &'a str = "loop";
    const SYNTAX_FOR: &'a str = "for";
    const SYNTAX_WHILE: &'a str = "while";
    const SYNTAX_MATCH: &'a str = "match";

    const SYNTAX_WORDS: [&'a str; 7] = [
        Self::SYNTAX_IF,    // if
        Self::SYNTAX_ELIF,  // elif
        Self::SYNTAX_ELSE,  // else
        Self::SYNTAX_LOOP,  // loop
        Self::SYNTAX_FOR,   // for
        Self::SYNTAX_WHILE, // while
        Self::SYNTAX_MATCH, // match
    ];
    const SYNTAX_WORDS_HEADS: [&'a str; 4] = [
        Self::SYNTAX_IF,    // if
        Self::SYNTAX_LOOP,  // loop
        Self::SYNTAX_FOR,   // for
        Self::SYNTAX_WHILE, // while
    ];
    const ESCAPECHAR: char = '\\';
    const FUNCTION: &'a str = "fn";
    const SEMICOLON: char = ';';
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

    const BLOCK_BRACE_OPEN: char = '{';
    const BLOCK_BRACE_CLOSE: char = '}';
    const BLOCK_PAREN_OPEN: char = '(';
    const BLOCK_PAREN_CLOSE: char = ')';
    const BLOCK_LIST_OPEN: char = '[';
    const BLOCK_LIST_CLOSE: char = ']';

    // fn code2vec(&self, code: &Vec<BaseElem>) -> Result<Vec<BaseElem>, &str>;

    fn resolve(&mut self) -> Result<(), ParserError>;

    fn code2_vec_pre_proc_func(&self, code: &String) -> Vec<BaseElem> {
        return code
            .chars()
            .map(|c| BaseElem::UnKnownElem(UnKnownBranch { contents: c }))
            .collect();
    }
}
