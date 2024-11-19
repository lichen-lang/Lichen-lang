// 式パーサーのテスト
//

extern crate colored;
extern crate lichen_lang;
use colored::*;
use lichen_lang::parser::core_parser::Parser;
use lichen_lang::parser::expr_parser::ExprParser;
use lichen_lang::abs::gen::Wasm_gen;
use lichen_lang::abs::ast::*;


#[test]
pub fn expr_test00() {
    let code = " !a&& !b";
    let string_code: String = String::from(code);
    println!("test case -> \"{}\"", code.cyan());
    let mut e_parser = ExprParser::new(string_code, 0, 0);

    if e_parser.resolve().is_err() {
        println!("ParseError occured");
    } else {
        println!("------------------------------");
        for i in e_parser.code_list {
            println!("{}", i.get_show_as_string());
        }
    }
}

#[test]
pub fn expr_test01() {
    let code = "func00(10, 123 + func01(a,b,c)) + 2 * x";
    let string_code: String = String::from(code);
    let mut e_parser = ExprParser::new(string_code, 0, 0);

    println!("test case -> {}", code.cyan());
    if e_parser.resolve().is_err() {
        println!("ParseError occured");
    } else {
        println!("{:#?}", e_parser.code_list);
        for i in e_parser.code_list {
            i.show();
        }
    }
}

#[test]
pub fn expr_test02() {
    let code = "(10+ 1) + 2 * x";
    let string_code: String = String::from(code);
    let mut e_parser = ExprParser::new(string_code, 0, 0);

    println!("test case -> {}", code);
    if e_parser.resolve().is_err() {
        println!("ParseError occured");
    } else {
        // println!("{:#?}", e_parser.code_list);
        for i in e_parser.code_list {
            i.show();
        }
    }
}

#[test]
pub fn expr_test03() {
    // let code = "tarai[1][2][3]";
    // let code = "tarai(1)(2)(3)";
    let code = "while  (0 < x) { 1 } else {0}(1)[2](3)";
    let string_code: String = String::from(code);
    let mut e_parser = ExprParser::new(string_code, 0, 0);

    println!("test case -> {}", code);
    if let Err(e) = e_parser.resolve() {
        println!("{:?}", e);
    } else {
        println!("{:#?}", e_parser.code_list);
        for i in e_parser.code_list {
            i.show();
        }
    }
}

#[test]
pub fn expr_test04() {
    let test_cases = vec![
        "// \"hello\"",
        "\"hello world\"",
        "
\"hello\"
/* \" */
// \"
",
        "\"\\\" <- quotation escape\"",
    ];
    for code in test_cases {
        println!("{}", "-".repeat(30));
        let string_code: String = String::from(code);
        let mut e_parser = ExprParser::new(string_code, 0, 0);

        println!("test case -> {}", code);
        if let Err(e) = e_parser.resolve() {
            println!("{}", format!("Parser Error {:?}", e).red());
        } else {
            println!("{:#?}", e_parser.code_list);
            for i in e_parser.code_list {
                i.show();
            }
        }
    }
}

