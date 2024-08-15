# tests

## テストすべき項目

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
cargo test --package lichen-lang --lib -- tests::expr_test01 --exact --show-output
```