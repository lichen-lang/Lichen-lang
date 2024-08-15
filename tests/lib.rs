extern crate colored;
extern crate lichen_lang;
mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::utils::insert_space;
    use colored::*;
    use lichen_lang::parser::core_parser::Parser;
    use lichen_lang::parser::expr_parser::ExprParser;

    #[test]
    fn test00() {
        println!("{}hello{}", " ".repeat(4), "@".repeat(4));
    }

    #[test]
    fn test01() {
        //
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
        println!("test case -> \"{}\"", code);
        let mut e_parser = ExprParser::new(string_code, 0, 0);

        if let Err(_) = e_parser.resolve() {
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
        let code = "func(10, 1) + 2 * x";
        let string_code: String = String::from(code);
        let mut e_parser = ExprParser::new(string_code, 0, 0);

        println!("test case -> {}", code);
        if let Err(_) = e_parser.resolve() {
            println!("ParseError occured");
        } else {
            for i in e_parser.code_list {
                i.show();
            }
        }
    }

    #[test]
    fn unit_test00() {
        let test_cases = vec![
            vec!["!", "a", "&&", "!", "b"],
            vec!["-", "10", "+", "20"],
            vec!["a", "**", "b", "**", "c"],
            vec!["a", "+", "b", "+", "c"],
        ];

        for test_case in test_cases {
            let mut ast_string = String::new();
            let mut ans_ast_string = String::new();
            let mut e_parser = ExprParser::new(test_case.join("").to_string(), 0, 0);
            if let Err(_) = e_parser.resolve() {
                println!("unexpected ParseError occured");
                assert!(false);
            } else {
                for i in e_parser.code_list {
                    ans_ast_string = format!("{}{}", ans_ast_string, i.get_show_as_string())
                }
                println!("{}", ans_ast_string);
            }

            // 同じように解釈されるべき文字列が同じように解釈されなかった場合Error!を出す
            for code in insert_space(test_case, 2) {
                let string_code: String = String::from(code.clone());
                let mut e_parser_unit = ExprParser::new(string_code, 0, 0);

                if let Err(_) = e_parser_unit.resolve() {
                    println!("ParseError occured");
                } else {
                    ast_string.clear();
                    for i in e_parser_unit.code_list {
                        ast_string = format!("{}{}", ast_string, i.get_show_as_string())
                    }
                    if ans_ast_string == ast_string {
                        println!(
                            "{} -> {}",
                            format!("test case -> \"{}\"", code),
                            "Ok".green()
                        );
                    } else {
                        // Error !
                        println!(
                            "{} -> {}",
                            format!("test case -> \"{}\"", code),
                            format!("{}{}", "Error!", ast_string).red()
                        );
                        assert!(false);
                    }
                }
            }
        }
    }
}
