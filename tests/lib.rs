extern crate colored;
extern crate lichen_lang;
mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::testutils::insert_space;
    use colored::*;
    use lichen_lang::abs::ast::*;
    use lichen_lang::parser::core_parser::Parser;
    use lichen_lang::parser::expr_parser::ExprParser;

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
                    ans_ast_string = format!("{}{}", ans_ast_string, i.get_show_as_string())
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
