extern crate colored;
extern crate lichen_lang;
mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::testutils::insert_space;
    use colored::*;
    use lichen_lang::abs::ast::*;
    use lichen_lang::abs::gen::Wasm_gen;
    use lichen_lang::parser::core_parser::Parser;
    use lichen_lang::parser::expr_parser::ExprParser;
    use lichen_lang::parser::stmt_parser::StmtParser;

    // expr tests

    #[test]
    fn expr_test00() {
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
    fn expr_test01() {
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
    fn expr_test02() {
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
    fn expr_test03() {
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
    fn expr_test04() {
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

    #[test]
    fn stmt_test00() {
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
            "
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


    #[test]
    fn gen_test00(){
        let test_cases = vec![
            "!a&&!bs" , 
            "-10+20"  , 
            // "a**b**c" ,
            "a+b+c"   ,
            "(a+bc)+(cde-defg)" ,
            "func(10,1)+2*x"    ,
            // "tarai(1)(2)(3)"    ,
            // "if (0 < x){ 1 } else {0} + 1" ,
            "c = !a&&!bs" , 
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

    #[test]
    fn unit_test00() {
        let test_cases = vec![
            vec!["!", "a", "&&", "!", "b"],  // !a&&!bs
            vec!["-", "10", "+", "20"],      // -10+20
            vec!["a", "**", "b", "**", "c"], // a**b**c
            vec!["a", "+", "b", "+", "c"],   // a+b+c
            vec!["(", "a", "+", "bc", ")", "+", "(", "cde", "-", "defg", ")"], // (a+bc)+(cde-defg)
            vec!["func", "(", "10", ",", "1", ")", "+", "2", "*", "x"], // func(10,1)+2*x
            vec!["tarai", "(", "1", ")", "(", "2", ")", "(", "3", ")"], // tarai(1)(2)(3)
            vec![
                "if", "(", "0", "<", "x", ")", "{", "1", "}", "else", "{", "0", "}", "+", "1",
            ], // if (0 < x){ 1 } else {0} + 1
        ];

        for test_case in test_cases {
            let mut ast_string = String::new();
            let mut ans_ast_string = String::new();
            let mut e_parser = ExprParser::new(test_case.join("").to_string(), 0, 0);
            if e_parser.resolve().is_err() {
                println!("unexpected ParseError occured");
                panic!()
            } else {
                for i in e_parser.code_list {
                    ans_ast_string.push_str(i.get_show_as_string().as_str());
                }
                println!("{}", ans_ast_string);
            }

            for n in 1..test_case.len() - 1 {
                // 同じように解釈されるべき文字列が同じように解釈されなかった場合Error!を出す
                for code in insert_space(test_case.clone(), n) {
                    let string_code: String = code.clone();
                    let mut e_parser_unit = ExprParser::new(string_code, 0, 0);

                    if e_parser_unit.resolve().is_err() {
                        println!("ParseError occured");
                    } else {
                        ast_string.clear();
                        for i in e_parser_unit.code_list {
                            ast_string = format!("{}{}", ast_string, i.get_show_as_string())
                        }
                        if ans_ast_string == ast_string {
                            println!("test case -> \"{}\" -> {}", code, "Ok".green());
                        } else {
                            // Error !
                            println!(
                                "test case -> \"{}\" -> {}",
                                code,
                                format!("{}{}", "Error!", ast_string).red()
                            );
                            panic!();
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn unit_test01() {
        let test_cases = vec![
            "a+1*2",
            "tarai[1][2][3]",
            "f(1,2,g(1,2,3))",
            "tarai(1)(2)(3)",
            "tarai(1)[2](3)",
            "tarai[1](2)[3]",
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
                    }
                }
            }
        }
    }
}
