# パーサーの開発

## expr_parser

## stmt_parser

stmtに分類されるトークン

```
let <name>(:<type>)( = <init>);
```

```
return ;
yield ;
continue ;
```

連続した式のうち特にsyntaxboxである方
```
1 + 2;
```


```
{
    <stmt>;
    <stmt>;
    <stmt>;
    <sxpr>   // <- <stmt>に非明示的に変換しても良い
    <stmt>;
}
```