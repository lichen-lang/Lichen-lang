# For Developers

## Lichenの基本的な構造

プログラムの詳細はプログラム自体である。このドキュメント自体はプログラムの大雑把な概要である

### ディレクトリ構造

各moduleのディレクトリ内に配置してある`README.md`がファイルのおおまかな説明を記述してある。（大多数はまだ工事中だ）

- src
  - abs
    
    中間表現の要素`token`のtrait、また列挙型が定義されている。

    [README](abs/README.md)
  - errors

    Lichenコンパイル時の一連の流れで発生したエラーコードとその時の処理を記述する。

    [README](errors/README.md)
  - parser

    トップレベルから呼び出され`token`を生成し始めると、その、`token`自体もまた`parser`を持っている。再帰的にプログラムを追っていく。

    [README](parser/README.md)
  - token

    Lichenの中間表現を構成する要素。parserをもって再帰的に自身を解析するものとそうでないものが存在する。

    [README](token/README.md)


## tests
テストすべき項目

- expr_prser

- state_parser

- ASTの構造

### expr test 00
`test case`
```
(10 + 1) + 2 * x
```

```bash
cargo test --package lichen-lang --lib -- tests::expr_test00 --exact --show-output
```

### expr test 01
`test case`
```
func(10,1) + 2 * x
```

```bash
cargo test --package lichen-lang --lib -- tests::expr_test00 --exact --show-output
```
