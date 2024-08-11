//use crate::parser::core::ExprParser;
mod abs;
mod errors;
mod parser;
mod token;

// test case
#[cfg(test)]
mod tests {

    use crate::parser::core_parser::Parser;
    use crate::parser::state_parser::StateParser;

    use crate::parser::expr_parser;

    #[test]
    fn test02() {
        println!("{}", "@".repeat(5));
    }

    // expr tests

    #[test]
    fn expr_test00() {
        let code = "(10 + 1) + 2 * x";
        let string_code: String = String::from(code);
        let mut e_parser = expr_parser::ExprParser::new(string_code, 0, 0);

        if let Err(_) = e_parser.resolve() {
            println!("ParseError occured");
        } else {
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

        if let Err(_) = e_parser.resolve() {
            println!("ParseError occured");
        } else {
            for i in e_parser.code_list {
                i.show();
            }
        }
    }
}
