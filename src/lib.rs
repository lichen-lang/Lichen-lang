// modules
mod abs;
mod errors;
mod parser;
mod token;

// test case
#[cfg(test)]
mod tests {

    use crate::parser::core_parser::Parser;
    use crate::parser::stmt_parser::StmtParser;

    use crate::parser::expr_parser;

    #[test]
    fn test00() {
        println!("{}hello{}", " ".repeat(4), "@".repeat(4));
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

        let code = "!a && !b";
        let string_code: String = String::from(code);
        println!("test case -> {}", code);
        let mut e_parser = expr_parser::ExprParser::new(string_code, 0, 0);

        if let Err(_) = e_parser.resolve() {
            println!("ParseError occured");
        } else {
            println!("------------------------------");
            for i in e_parser.code_list {
                i.show();
            }
        }
    }

    #[test]
    fn expr_test01() {
        let code = "func(10, 1) + 2 * x";
        let string_code: String = String::from(code);
        let mut e_parser = expr_parser::ExprParser::new(string_code, 0, 0);

        println!("test case -> {}", code);
        if let Err(_) = e_parser.resolve() {
            println!("ParseError occured");
        } else {
            for i in e_parser.code_list {
                i.show();
            }
        }
    }
}
