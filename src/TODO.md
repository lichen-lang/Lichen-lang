# TODO

- パース関連のコンパイラ開発者向けエラーの細分化

- `expr` `stmt`といった形のもう一つ抽象的なレイヤーを追加する
  - `expr` 式
  - `stmt` 文 

- syntaxの途中にコメントがあったりする場合でも正しくパースされるようにする

```
if /*this is test*/(expr) // hello world
{

}
```
