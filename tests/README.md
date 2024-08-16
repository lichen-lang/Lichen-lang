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
func(10,1) + 2 * x
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
### unit test00

expr_parserが正常に動作するかを確かめるテスト
```bash
cargo test --package lichen-lang --test lib -- tests::unit_test00 --exact --show-output
```