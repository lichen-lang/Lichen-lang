// core
pub mod core_parser;

// 以下はcore_parser traitを実装している
// core_parser.parser for *
pub mod comma_parser; // カンマ区切りの引数(いずれ、パターンになる可能性がある)
pub mod expr_parser; // experiment parser
pub mod stmt_parser; // statement parser
pub mod type_parser; // typing parser
