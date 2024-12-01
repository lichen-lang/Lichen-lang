use crate::abs::ast::*;
use crate::abs::gen::Wasm_gen;
use crate::errors::generate_errors::GenerateError;
use crate::gen::wasm::MEMORY_SPACE_NAME;


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
            "=" => assembly_text.push_str(
                &equal_gen_wasm(l_expr, r_expr)?), // equal
            "+=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "+=")?),
            "-=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "-=")?),
            "*=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "*=")?),
            "/=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "/=")?),
            "%=" => assembly_text.push_str(&ref_aequal_gen_wasm(l_expr, r_expr, "%=")?),
            "+" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.add\n")?),
            "-" => assembly_text.push_str(
                &sub_gen_wasm(l_expr, r_expr)?) , // subtract
            "*" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.mul\n")?),
            "/" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.div\n")?) ,
            "%" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.rem_s\n")?) ,
            "&&" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.and\n")?),
            "||" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.or\n")?),
            "!" => assembly_text.push_str(
                &not_gen_wasm(l_expr, r_expr)?),  // xor を使ってnotを再現している
            //
            "==" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.eq\n")?) ,
            "!=" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.ne\n")?) ,
            // 大小には`signed` `unsigned`がある
            "<" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.lt_s\n")?) ,
            ">" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.gt_s\n")?) ,
            "<=" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.le_s\n")?) ,
            ">=" => assembly_text.push_str(
                &normal_ope_gen_wasm(l_expr, r_expr, "i32.ge_s\n")?) ,
            _ => return Err(GenerateError::InvalidOperation),
        }
        Ok(assembly_text)
    }
}


fn equal_gen_wasm(l_expr:&ExprElem, r_expr:&ExprElem) -> Result<String, GenerateError>{
    let mut assembly_text = String::default();
    let mut r_assembly_text = String::default();

    if let ExprElem::ItemElem(item_b) = r_expr {
        r_assembly_text = item_b.generate_wasm()?;
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = l_expr {
        // とりあえず、パターンなどを考えず、一つの変数に値を代入する
        // 場合のみの実装
        if let ExprElem::WordElem(word_b) = &item_b.contents[0]{
            // 普通の変数に代入するのと同じ
            // a = 1;
            // のようなケース
            assembly_text.push_str(&r_assembly_text);
            assembly_text.push_str(&format!("local.set ${}\n" , word_b.contents));
        } else if let ExprElem::ListElem(list_b) = &item_b.contents[0] {
            // pass
            // TODO
            // a[0] = 1;のようなケース
            // ^
            // 呼び出す対象が名前の場合
            // ```
            // <list elem> = <r_expr>
            // ```

            if let ExprElem::WordElem(word_b) = &*list_b.name {
                if word_b.contents == MEMORY_SPACE_NAME {
                    // 特別なケース、メモリに直接アクセスするための方法を提供する
                    // ```
                    // __mem[0] = 0;
                    // ```
                    assembly_text.push_str(&list_b.generate_contents_wasm()?);
                    assembly_text.push_str(&r_assembly_text);
                    assembly_text.push_str("i32.store\n");
                } else {
                    // 通常のケース
                    // ```
                    // a[0] = 0;
                    // ```
                    todo!()
                }
            } else {
                // 呼び出しの対象が名前ではない場合
                // ```
                // lst[0][0]
                // ^^^^^^
                // ```
                todo!()
            }
        }
        else {
            // word 以外がパターンに渡された場合
            return Err(GenerateError::InvalidleftPattern);
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    Ok(assembly_text)
}

pub fn ref_aequal_gen_wasm(l_expr:&ExprElem, r_expr:&ExprElem, ope:&str) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    let r_assembly_text  :String;
    let getter_assembly_text :String;
    let setter_assembly_text :String;

    // a += 1;
    // ^    ^
    // a = a + 1;
    if let ExprElem::ItemElem(item_b) = r_expr{
        r_assembly_text = item_b.generate_wasm()?;
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = l_expr{
        // 左は式ではなくパターンの処理をする必要があります
        if let ExprElem::WordElem(word_b) = &item_b.contents[0]{
            // pass
            setter_assembly_text = format!("local.set ${}\n", word_b.contents); // setter
            getter_assembly_text = format!("local.get ${}\n", word_b.contents); // setter
        } else {
            println!("まだサポートしていない書き方です"); // TODO
            todo!()
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    // ```wat
    // ;; 10 - 3
    // local.get $a
    // i32.const 1
    // i32.sub ;; sub 
    // local.set $a
    // ```
    assembly_text.push_str(&getter_assembly_text);
    assembly_text.push_str(&r_assembly_text);
    match ope{
        "+=" => assembly_text.push_str("i32.add\n"),
        "-=" => assembly_text.push_str("i32.sub\n"),
        "*=" => assembly_text.push_str("i32.mul\n"),
        "/=" => assembly_text.push_str("i32.div\n"),
        "%=" => assembly_text.push_str("i32.rem_s\n"),
        _ => 
            return Err(GenerateError::Deverror),
    }
    assembly_text.push_str(&setter_assembly_text);
    Ok(assembly_text)
}

/// ふたつの引数を両端からとる"普通の"演算子の生成
fn normal_ope_gen_wasm(l_expr:&ExprElem, r_expr:&ExprElem, ope_string:&str)-> Result<String, GenerateError>{
    let mut assembly_text = String::default();
    if let ExprElem::ItemElem(item_b) = l_expr{
        assembly_text.push_str(&item_b.generate_wasm()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = r_expr{
        assembly_text.push_str(&item_b.generate_wasm()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    assembly_text.push_str(ope_string);
    Ok(assembly_text)
}


/// 前置記法の場合わけが必要なケース("-"の場合)
fn sub_gen_wasm(l_expr:&ExprElem, r_expr:&ExprElem) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    if let ExprElem::ItemElem(item_b) = l_expr{
        if item_b.has_no_elem() {
            assembly_text.push_str("i32.const 0\n");
        } else {
            assembly_text.push_str(&item_b.generate_wasm()?);
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = r_expr{
        assembly_text.push_str(&item_b.generate_wasm()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    assembly_text.push_str("i32.sub\n");
    Ok(assembly_text)
}


/// 前置記法の場合わけが必要なケース("!"の場合)
fn not_gen_wasm(l_expr:&ExprElem, r_expr:&ExprElem) -> Result<String, GenerateError> {
    let mut assembly_text = String::default();
    if let ExprElem::ItemElem(item_b) = l_expr{
        if item_b.has_no_elem() {
            assembly_text.push_str("i32.const 1\n");
        } else {
            assembly_text.push_str(&item_b.generate_wasm()?);
        }
    } else {
        return Err(GenerateError::Deverror);
    }
    if let ExprElem::ItemElem(item_b) = r_expr{
        assembly_text.push_str(&item_b.generate_wasm()?);
    } else {
        return Err(GenerateError::Deverror);
    }
    assembly_text.push_str("i32.xor\n");
    Ok(assembly_text)
}


