extern crate lichen_lang;

mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::utils::insert_space;
    use lichen_lang::parser::core_parser::Parser;
    use lichen_lang::parser::expr_parser::ExprParser;
    // use crate::test::utils::{combinations, insert_space, CombinationIter};

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
        let mut b = vec![3, 4, 5];

        a.extend(b);
        println!("{:?}", a);
    }

    // expr tests

    #[test]
    fn expr_test00() {
        // let code = "(10 + 1) + 2 * x";
        // let string_code: String = String::from(code);
        // println!("test case -> {}", code);
        // let mut e_parser = expr_parser::ExprParser::new(string_code, 0, 0);

        // if let Err(_) = e_parser.resolve() {
        //     println!("ParseError occured");
        // } else {
        //     for i in e_parser.code_list {
        //         i.show();
        //     }
        // }

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
        let a = vec!["!", "a", "&&", "!", "b"];
        let mut str_tmp: Option<String> = None;

        let mut ast_string = String::new();
        let mut ans_ast_string = String::new();
        let mut e_parser = ExprParser::new("!a&&!b".to_string(), 0, 0);
        if let Err(_) = e_parser.resolve() {
            println!("ParseError occured");
        } else {
            for i in e_parser.code_list {
                ans_ast_string = format!("{}{}", ans_ast_string, i.get_show_as_string())
            }
            println!("{}", ans_ast_string);
            str_tmp = Some(ans_ast_string.clone());
        }

        // 同じように解釈されるべき文字列が同じように解釈されなかった場合Error!を出す
        for code in insert_space(a, 2) {
            let string_code: String = String::from(code.clone());
            println!("test case -> \"{}\"", code);
            let mut e_parser = ExprParser::new(string_code, 0, 0);

            if let Err(_) = e_parser.resolve() {
                println!("ParseError occured");
            } else {
                // println!("------------------------------");
                ast_string.clear();
                for i in e_parser.code_list {
                    ast_string = format!("{}{}", ast_string, i.get_show_as_string())
                }
                // println!("{}", ast_string);
                if let Some(pre_str) = &str_tmp {
                    assert!(ans_ast_string == ast_string);
                    if ans_ast_string == ast_string {
                        println!("Ok");
                    } else {
                        println!("Error!\n{}", ast_string);
                    }
                } else {
                    str_tmp = Some(ast_string.clone());
                }
            }
        }
    }
}
