# tests

## テストすべき項目

- expr_prser

- state_parser

- ASTの構造チェック

### expr test 00
`test case`
```
(10 + 1) + 2 * x
```

```bash
cargo test --package lichen-lang --test lib -- tests::expr_test00 --exact --show-output 
```

### expr test 01
`test case`
```
func00(10, 123 + func01(a,b,c)) + 2 * x
```

```bash
cargo test --package lichen-lang --test lib -- tests::expr_test01 --exact --show-output
```

### expr test 02
debug trait test
```
(10 + 1) + 2 * x
```

```bash
cargo test --package lichen-lang --test lib -- tests::expr_test02 --exact --show-output
```

### expr test 03

```
tarai[1][2][3]
```

```bash
cargo test --package lichen-lang --test lib -- tests::expr_test03 --exact --show-output
```

### unit test00

expr_parserが正常に動作するかを確かめるテスト00
```bash
cargo test --package lichen-lang --test lib -- tests::unit_test00 --exact --show-output
```

### unit test01

expr_parserが正常に動作するかを確かめるテスト01
`callable` `subscriptable`なコードが正常な動作をするかどうかのテスト

```bash
cargo test --package lichen-lang --test lib -- tests::unit_test01 --exact --show-output
```

## テストの走るタイミング

テストはローカル環境で上のコマンドで実行することができる。

developまたは,masterブランチへPRを送ったとき。また、それぞれにマージされたとき以下のテストが実行される。

```bash
cargo test
```