# For Developers

## `core_parser.rs`

- Parser trait
  すべてのパーサーに共通するメソッドの宣言と、定数の宣言

## parserの構造

- `expr_parser.rs`
  式をパースするためのparser

- `stmt_parser.rs`
  文をパースするためのparser

- `comma_parser.rs`
  カンマ区切りのコードのためのparser