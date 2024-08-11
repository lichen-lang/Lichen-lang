
## tests
テストすべき項目

- expr_prser

- state_parser

- ASTの構造

### test00
```bash
cargo test --package lichen-lang --lib -- tests::test00 --exact --show-output
```

### test01
```bash
cargo test --package lichen-lang --lib -- tests::test01 --exact --show-output
```

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

### expr test 02
resolve2 function test
```bash
cargo test --package lichen-lang --lib -- tests::expr_test00 --exact --show-output
```