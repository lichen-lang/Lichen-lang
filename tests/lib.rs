extern crate colored;
extern crate lichen_lang;


mod utils;
mod test_set; 

use crate::test_set::*;

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
}
