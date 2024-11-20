// 文のテスト
// stmt


extern crate lichen_lang;
use lichen_lang::parser::core_parser::Parser;
use lichen_lang::parser::stmt_parser::StmtParser;
use lichen_lang::abs::gen::Wasm_gen;
use lichen_lang::abs::ast::*;


#[test]
pub fn stmt_test00() {
        let mut ans_ast_string = String::new();
        let test_cases = vec![
            "
		let mut a = 1 + 1;
		let b = 1 + 1;
                if (a < b) // hello world
                {
                    print(a, b);
                };
		b = 1 + 1;
	    ",
            "
		let mut a = 1 + 1;
		let b = 1 + 1;
                if (a < b){
                    print(a, b);
                }
                elif (a < b){
                    print(a, b);
                };
		b = 1 + 1;
            ",
            "
            let a = tarai(tarai(x - 1,  y, z), tarai(y - 1, z, x), tarai(z - 1, x, y));
            ",
            "let a: i32 = 1+2;",
            "let a: (i32, i32) -> i32 -> i32 = f();",

        ];
        for test_case in test_cases{
            let mut s_parser = StmtParser::new(test_case.to_string(), 0,0);
            println!("----------------------------------------------------------------");
            if let Err(e) = s_parser.resolve()
            {
                println!("unexpected ParseError occured");
                println!("{:?}", e);
                panic!()
            }
            else
            {
                for i in s_parser.code_list{
                    ans_ast_string.push_str(i.get_show_as_string().as_str());
                }
                println!("{}", ans_ast_string);
                ans_ast_string.clear();
            }
        }
    }

