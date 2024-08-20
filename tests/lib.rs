extern crate colored;
extern crate lichen_lang;
mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::testutils::insert_space;
    use colored::*;
    use lichen_lang::parser::core_parser::Parser;
    use lichen_lang::parser::expr_parser::ExprParser;

    #[test]
    fn test00() {
        println!("{}hello{}", " ".repeat(4), "@".repeat(4));
    }

    fn func00() -> Result<i32, i32> {
        Err(42)
    }

    fn func01() -> Result<i32, i32> {
        Err(func00()?)
    }

    #[test]
    fn test01() {
        let a = func01();
        match a {
            Ok(a) => println!("OK {}", a),
            Err(e) => println!("ERR {}", e),
        }
    }

    #[test]
    fn test02() {
        let mut a = vec![0, 1, 2];
        let b = vec![3, 4, 5];

        a.extend(b);
        println!("{:?}", a);
    }

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
            // println!("{:#?}", e_parser.code_list);
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
            println!("{:#?}", e_parser.code_list);
        }
    }

    #[test]
    fn expr_test03() {
        let code = "tarai(1)(2)(3)";
        let string_code: String = String::from(code);
        let mut e_parser = ExprParser::new(string_code, 0, 0);

        println!("test case -> {}", code);
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
    fn unit_test00() {
        let test_cases = vec![
            vec!["!", "a", "&&", "!", "b"],  // !a&&!bs
            vec!["-", "10", "+", "20"],      // -10+20
            vec!["a", "**", "b", "**", "c"], // a**b**c
            vec!["a", "+", "b", "+", "c"],   // a+b+c
            vec!["(", "a", "+", "bc", ")", "+", "(", "cde", "-", "defg", ")"], // (a+bc)+(cde-defg)
            vec!["func", "(", "10", ",", "1", ")", "+", "2", "*", "x"], // func(10,1)+2*x
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
}
