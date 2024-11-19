// 生成テスト
// 生成されたアセンブリが正しいかチェックする。
//

extern crate lichen_lang;
use lichen_lang::parser::core_parser::Parser;
use lichen_lang::parser::expr_parser::ExprParser;
use lichen_lang::abs::gen::Wasm_gen;
use lichen_lang::abs::ast::*;

// install wasmer
use wasmer::{Store, Module, Instance, Value, imports};
use anyhow;


/// https://crates.io/crates/wasmer/
pub fn run_wasm() -> anyhow::Result<()> {
    let module_wat = r#"
    (module
    (type $t0 (func (param i32) (result i32)))
    (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        local.get $p0
        i32.const 1
        i32.add))
    "#;

    let mut store = Store::default();
    let module = Module::new(&store, &module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));

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
                            match a.generate(){
                                Ok(s) => {
                                    println!("{}", s);
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

