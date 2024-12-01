use crate::abs::ast::*;
use crate::abs::gen::Wasm_gen;
use crate::errors::parser_errors::ParserError;
use crate::parser::core_parser::Parser;
use crate::parser::expr_parser::ExprParser;
use crate::parser::stmt_parser::StmtParser;
use crate::errors::generate_errors::GenerateError;


/// # SyntaxBranch
/// `if` `elif` `else` `while` `loop` `for`などのデータを扱うstruct
/// resolve_selfはそれぞれ
/// `()`で格納されているデータに関しては`ParenBlockBranch`をnormalで呼び出す
/// `{}`で格納されているデータに関しては`BlockBranch`のパーサに丸投げする。
/// 当然、全てのブロックが何かで満たされるわけではないので注意
#[derive(Clone, Debug)]
pub struct SyntaxBranch {
    pub name: String,
    pub expr: Vec<ExprElem>,
    pub contents: Vec<StmtElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl ASTBranch for SyntaxBranch {
    fn show(&self) {
        println!("{}", self.name);
        println!("expr");
        for i in &self.expr {
            i.show()
        }
        println!("{}{{", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            i.show()
        }
        println!("{}}}", " ".repeat(self.depth as usize * 4));
    }

    fn get_show_as_string(&self) -> String {
        let mut expr_string = String::new();

        for i in &self.expr {
            expr_string = format!("{}{}", expr_string, i.get_show_as_string());
        }
        expr_string = format!("expr({})", expr_string);

        let mut block_string = format!("{}{{", " ".repeat(self.depth as usize * 4));
        for i in &self.contents {
            block_string = format!("{}{}\n", block_string, i.get_show_as_string())
        }
        block_string = format!("{}{}}}", block_string, " ".repeat(self.depth as usize * 4));
        format!("{}{}{}", self.name, expr_string, block_string)
    }
}

impl RecursiveAnalysisElements for SyntaxBranch {
    fn resolve_self(&mut self) -> Result<(), ParserError> {
        let mut e_parser =
            ExprParser::create_parser_from_vec(
                self.expr.clone(), 
                self.depth,
                self.loopdepth
            );
        e_parser.resolve()?;
        self.expr = e_parser.code_list;
        let mut s_parser =
            StmtParser::create_parser_from_vec(
                self.contents.clone(),
                self.depth,             
                if self.name == "while" || self.name == "for" {
                    self.loopdepth + 1
                } else {
                    self.loopdepth
                }
                );
        s_parser.resolve()?;
        self.contents = s_parser.code_list;
        Ok(())
    }
}

impl SyntaxBranch {
    pub fn generate_wasm(&self, head_name: &str) -> Result<String, GenerateError> {
        let mut assembly_text = String::new();
        match head_name{
            "if" => {
                assembly_text.push_str( &wasm_if_gen(self)?);
            }
            "while" => {
                assembly_text.push_str(&wasm_while_gen(self)?);
            }
            "for" => {
                todo!()
            }
            _ => {
                return Err(GenerateError::Deverror);
            }
        }
        Ok(assembly_text)
    }
}


fn wasm_if_gen(if_state:&SyntaxBranch) -> Result<String, GenerateError>{
    let mut assembly_text = String::new();

    match &*if_state.name{
        "if" => {
            if if_state.expr.is_empty(){
                // TODO
                // if の条件式が空はおかしいというerror
                // を返す
                todo!()
            } else if if_state.expr.len() == 1{
                if let ExprElem::FuncElem(func_b) = &if_state.expr[0]{
                    // 式を展開
                    assembly_text.push_str(&func_b.generate_wasm()?);
                    assembly_text.push_str("if\n");
                    // 文をwasmように展開
                    assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
                }else if let ExprElem::ParenBlockElem(paren_b) = &if_state.expr[0]{
                    // 式を展開
                    assembly_text.push_str(&paren_b.generate_wasm()?);
                    assembly_text.push_str("if\n");
                    // 文をwasmように展開
                    assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
                } else{
                    // TODO
                    // if の条件式が関数ではないのはおかしいというerror
                    // を返す
                    todo!()
                }
            } else {
                // ここで、exprelemが複数あるのはおかしい
                return Err(GenerateError::Deverror);
            }
        }
        "elif" => {
            if if_state.expr.is_empty(){
                // TODO
                // else if の条件式が空はおかしいというerror
                // を返す
                todo!()
            } else if if_state.expr.len() == 1{
                if let ExprElem::FuncElem(func_b) = &if_state.expr[0]{
                     // 式を展開
                    assembly_text.push_str("else\n");
                    assembly_text.push_str(&func_b.generate_wasm()?);
                    assembly_text.push_str("if\n");
                    // 文をwasmように展開
                    assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
                } else if let ExprElem::ParenBlockElem(paren_b) = &if_state.expr[0]{
                    // 式を展開
                    assembly_text.push_str("else\n");
                    assembly_text.push_str(&paren_b.generate_wasm()?);
                    assembly_text.push_str("if\n");
                    // 文をwasmように展開
                    assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
                } else {
                    // TODO
                    // else if の条件式が関数ではないのはおかしいというerror
                    // を返す
                    todo!()
                }
            } else {
                // ここで、exprelemが複数あるのはおかしい
                return Err(GenerateError::Deverror);
            }
        }
        "else" => {
            if if_state.expr.is_empty(){
                assembly_text.push_str("else\n");
                assembly_text.push_str(&wasm_stmt_gen(&if_state.contents)?);
            } else {
                // else 説に条件式を設定しているのはおかしいというerror
                todo!()
            }
        }
        _ => {
            return Err(GenerateError::Deverror)
        }
    }
    Ok(assembly_text)
}


fn wasm_while_gen(while_state:&SyntaxBranch) -> Result<String, GenerateError> {
    use crate::gen::wasm::{BLOCK_ADDR, LOOP_ADDR};
    let mut assembly_text = String::default();
    
    // pass
    match &*while_state.name{
        "while" => {
            let loop_addr = format!("{}{}", LOOP_ADDR, while_state.loopdepth);
            let block_addr = format!("{}{}", BLOCK_ADDR, while_state.loopdepth);

            assembly_text.push_str(&format!("loop ${}\n", loop_addr));
            assembly_text.push_str(&format!("block ${}\n", block_addr));
            if while_state.expr.is_empty() {
                // TODO
                // if の条件式が空はおかしいというerror
                // を返す
                todo!()
            } else if while_state.expr.len() == 1 {
                if let ExprElem::FuncElem(func_b) = &while_state.expr[0]{
                    // 式を展開
                    assembly_text.push_str(&func_b.generate_wasm()?);
                }else if let ExprElem::ParenBlockElem(paren_b) = &while_state.expr[0]{
                    // 式を展開
                    assembly_text.push_str(&paren_b.generate_wasm()?);
                } else{
                    // TODO
                    // if の条件式が関数またはカッコではないのはおかしいというerror
                    // を返す
                    todo!()
                }
            } else {
                // ここで、exprelemが複数あるのはおかしい
                return Err(GenerateError::Deverror);
            }
            // not
            assembly_text.push_str("i32.const 1\n");
            assembly_text.push_str("i32.xor\n");
            assembly_text.push_str(&format!("br_if ${}\n", block_addr));
            assembly_text.push_str(&wasm_stmt_gen(&while_state.contents)?);
            assembly_text.push_str(&format!("br ${}\n", loop_addr));
            assembly_text.push_str("end\n");
            assembly_text.push_str("end\n");
        }
        _ => {
            // dev error 
            return Err(GenerateError::Deverror);
        }
    }
    Ok(assembly_text)
}


fn wasm_stmt_gen(stmt_list: &[StmtElem]) -> Result<String, GenerateError>{
    let mut assembly_text = String::default();
    for s in stmt_list{
        if let StmtElem::ExprElem(expr_b) = s {
            assembly_text.push_str(&expr_b.generate_wasm()?);
        }else if let StmtElem::Special(control_b) = s{
            assembly_text.push_str(&control_b.generate_wasm()?);
        } else if let StmtElem::CommentElem(comment_b) = s {
            // pass
            assembly_text.push_str("");
        }
        else{
            // これ以外のわたしが認識していない場合
            // コメントだった場合について実装する
            //
            todo!()
        }
    }
    Ok(assembly_text)
}

