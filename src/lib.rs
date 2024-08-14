// modules
pub mod abs;
pub mod errors;
pub mod parser;
mod test;
pub mod token;

// test case
#[cfg(test)]
mod tests {

    use crate::parser::core_parser::Parser;
    use crate::parser::stmt_parser::StmtParser;
    use crate::test::utils::{combinations, CombinationIter};

    use crate::parser::expr_parser;

    #[test]
    fn test00() {
        println!("{}hello{}", " ".repeat(4), "@".repeat(4));
    }

    #[test]
    fn test01() {
        let a = vec!["!", "a", "&&", "!", "b"];
        for i in combinations(a, 2) {
            println!("{:?}", i.join(""));
        }
        let a = vec![1, 2, 3];
        for i in combinations(a, 2) {
            println!("{:?}", i);
        }
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

        let code = "-10 + 20";
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
