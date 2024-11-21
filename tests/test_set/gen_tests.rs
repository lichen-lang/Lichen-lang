// 生成テスト
// 生成されたアセンブリが正しいかチェックする。
//

extern crate lichen_lang;
use colored::{Color, Colorize};
use lichen_lang::parser::core_parser::Parser;
use lichen_lang::parser::expr_parser::ExprParser;
use lichen_lang::abs::gen::Wasm_gen;
use lichen_lang::abs::ast::*;

// install wasmer
use wasmer::{Store, Module, Instance, Value, imports};
use anyhow;

/// `a:i32` `b:i32`の２つの引数をうけとり一つ返り値を返却するような式をテストする
///
pub fn wasm_test_2_args_1_return(
        wasm_code:&str,
        test_args_set:&[(i32, i32)],
        ans:&[i32]
    ) -> anyhow::Result<()> {
    let module_wat = &format!(r#"
(module
(type $t0 (func
(param i32)
(param i32)
(result i32)
))
(func $test (export "test") (type $t0)
(param $a i32)
(param $b i32)
(result i32)
;; -- start --
{}
;; --  end  --
))
    "#,
    wasm_code
    );
    println!("--- wasm code ---");
    println!("{}", module_wat);

    let mut store = Store::default();
    let module = Module::new(&store, module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let wasm_test = instance.exports.get_function("test")?;

    for (i, &(arg1,arg2)) in test_args_set.iter().enumerate(){
        let result = wasm_test.call(&mut store, &[Value::I32(arg1), Value::I32(arg2)])?;
        println!(
            "a:{}, b:{}, result: {} ans: {} -> {}",
            arg1,
            arg2,
            result[0],
            ans[i],
            if result[0] == Value::I32(ans[i])
            {
                "Ok".color(Color::Green)
            }
            else
            {
                "Failed".color(Color::Red)
            }
            );
        assert_eq!(result[0], Value::I32(ans[i]));
    }
    Ok(())
}

/// https://crates.io/crates/wasmer/
/// wasmerの使い方の例
#[test]
pub fn run_wasm() -> anyhow::Result<()> {
    let module_wat = r#"
    (module
    (type $t0 (func (param i32) (result i32)))
    (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        local.get $p0
        i32.const 1
        i32.add
    ))
    "#;

    let mut store = Store::default();
    let module = Module::new(&store, module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;

    for i in 0..10{
        let result = add_one.call(&mut store, &[Value::I32(i)])?;
        println!("i:{} result: {}", i,result[0]);
        // assert_eq!(result[0], Value::I32(i + 1));
    }
    Ok(())
}


#[test]
pub fn gen_test00(){
    let test_cases = vec![
        "!a&&!b" , 
        "a != b" , 
        "0 <= a && a <= 10" , 
        "-10+20"  , 
        // "a**b**c" ,
        "a+b+c"   ,
        "(a+bc)+(cde-defg)" ,
        "func(10,1)+2*x"    ,
        // "tarai(1)(2)(3)"    ,
        // "if (0 < x){ 1 } else {0} + 1" ,
        "c = !a&&!b" , 
        "d = -10+20"  , 
        // "a**b**c" ,
        "e = a+b+c"   ,
        "f = (a+bc)+(cde-defg)" ,
        "g = func(10,1)+2*x"    ,
        "var = (10 - 1) + 2 * ((1 + 4) * 5)", // 59
        "tarai(tarai(x - 1,  y, z), tarai(y - 1, z, x), tarai(z - 1, x, y))",
    ];

    for code in test_cases {
        let string_code: String = String::from(code);
        let mut e_parser = ExprParser::new(string_code, 0, 0);
        println!("------------------------------------------");
        println!("test case -> {}", code);
        match e_parser.resolve() {
            Err(e) => println!("{:?}", e),
            Ok(_) => {
                for i in e_parser.code_list {
                    i.show();
                    println!("--- wasm code ---");
                    match &i{
                        ExprElem::FuncElem(a) => {
                            match a.generate_wasm(){
                                Ok(wasm_code) => {
                                    // 
                                    println!("{}", wasm_code);
                                }
                                Err(e) => {
                                    println!("wasm生成中にerrorが発生しました");
                                    println!("{:?}", e);
                                }
                            }
                        }
                        _ => {
                            println!("func 要素ではありませんでした");
                            continue;
                        }
                    }
                }
            }
        }
    }
}


/// return がboolになるような2引数のケースをwasmerで実際に実行して、テストします。
/// 
#[test]
pub fn gen_test01(){
    let test_cases = ["!a&&!b" , 
        "!(a||b)" , 
        "a != b" , 
        "a * b != a + b" , 
        "-a <= b && b <= a"];

    let arg_set:Vec<Vec<(i32, i32)>> = vec![
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // !a&&!b
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // !(a||b)
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // a != b
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // a * b != a + b
        vec![(0, 0), (0, 1), (1, 0), (1, 1)], // -a <= b && b <= a
    ];

    let ans:Vec<Vec<i32>> = vec![
        vec![1, 0, 0, 0], // !a&&!b
        vec![1, 0, 0, 0], // !(a||b)
        vec![0, 1, 1, 0], // a != b
        vec![0, 1, 1, 1], // a * b != a + b
        vec![1, 0, 1, 1], // -a <= b && b <= a
    ];

    for (i, &code) in test_cases.iter().enumerate() {
        let string_code: String = String::from(code);
        let mut e_parser = ExprParser::new(string_code, 0, 0);
        println!("------------------------------------------");
        println!("test case -> {}", code);
        match e_parser.resolve() {
            Err(e) => println!("{:?}", e),
            Ok(_) => {
                for expr in e_parser.code_list {
                    expr.show();
                    match &expr{
                        ExprElem::FuncElem(a) => {
                            match a.generate_wasm(){
                                Ok(wasm_code) => {
                                    let _ = wasm_test_2_args_1_return(&wasm_code,  &arg_set[i], &ans[i]);
                                }
                                Err(e) => {
                                    println!("wasm生成中にerrorが発生しました");
                                    println!("コンパイラに何らかの問題があります");
                                    println!("{:?}", e);
                                }
                            }
                        }
                        _ => {
                            println!("func 要素ではありませんでした");
                            continue;
                        }
                    }
                }
            }
        }
    }
}
