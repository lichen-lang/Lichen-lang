use crate::abs::ast::*;
use crate::abs::gen::Wasm_gen;
use crate::errors::generate_errors::GenerateError;

/// #OperatorBranch
/// 全ての演算子
///
#[derive(Clone, Debug)]
pub struct OperatorBranch {
    pub ope: String,
    pub depth: isize,
}

impl ASTBranch for OperatorBranch {
    fn show(&self) {
        println!(
            "{}Operator({})\n",
            " ".repeat(self.depth as usize * 4),
            self.ope
        );
    }

    fn get_show_as_string(&self) -> String {
        format!(
            "{}Operator({})\n",
            " ".repeat(self.depth as usize * 4),
            self.ope
        )
    }
}


impl OperatorBranch{
    pub fn generate_wasm(&self, l_expr:&ExprElem,r_expr:&ExprElem) -> Result<String, GenerateError> {
        // 最初はi32向のみ対応
        // ここで送られて来るデータが本当に文字列でいいのか考える
        let mut assembly_text = String::default();
        match &*self.ope{
            "=" => assembly_text.push_str( &equal_gen_wasm(l_expr, r_expr)?),
            "+" => assembly_text.push_str( &normal_ope_gen_wasm(l_expr, r_expr, "i32.add\n")?),
            "-" => assembly_text.push_str( &sub_gen_wasm(l_expr, r_expr)?) ,
            "*" => assembly_text.push_str( &normal_ope_gen_wasm(l_expr, r_expr, "i32.mul\n")?),
            "/" => assembly_text.push_str( &normal_ope_gen_wasm(l_expr, r_expr, "i32.div\n")?) ,
            _ => return Err(GenerateError::InvalidOperation),
        }
        Ok(assembly_text)
    }
}


fn equal_gen_wasm(l_expr:&ExprElem, r_expr:&ExprElem) -> Result<String, GenerateError>{
    let mut assembly_text = String::default();
    if let ExprElem::ItemElem(item_b) = r_expr{
            assembly_text.push_str(&item_b.generate()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = l_expr{
        // とりあえず、パターンなどを考えず、一つの変数に値を代入する
        // 場合のみの実装
        if let ExprElem::WordElem(word_b) = &item_b.contents[0]{
            assembly_text .push_str(&format!("local.set ${}\n" , word_b.contents));
        } else {
            // word 以外がパターンに渡された場合
            return Err(GenerateError::InvalidleftPattern);
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    Ok(assembly_text)
}


fn normal_ope_gen_wasm(l_expr:&ExprElem, r_expr:&ExprElem, ope_string:&str)-> Result<String, GenerateError>{
    let mut assembly_text = String::default();
    if let ExprElem::ItemElem(item_b) = l_expr{
        assembly_text.push_str(&item_b.generate()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = r_expr{
        assembly_text.push_str(&item_b.generate()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    assembly_text.push_str(ope_string);
    Ok(assembly_text)
}


fn sub_gen_wasm(l_expr:&ExprElem, r_expr:&ExprElem)-> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    if let ExprElem::ItemElem(item_b) = l_expr{
        if item_b.has_no_elem() {
            assembly_text.push_str("i32.const 0\n");
        } else {
            assembly_text.push_str(&item_b.generate()?);
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = r_expr{
        assembly_text.push_str(&item_b.generate()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    assembly_text.push_str("i32.sub\n");
    Ok(assembly_text)
}
