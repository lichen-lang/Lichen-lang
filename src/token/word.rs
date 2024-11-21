use crate::abs::ast::*;
use crate::errors::generate_errors::GenerateError;


/// # WordBranch
/// 単語を格納するためのstruct
/// ASTAreaBranchを実装しないため`resolve_self`メソッドを持たない

#[derive(Clone, Debug)]
pub struct WordBranch {
    pub contents: String,
    pub depth: isize,
    pub loopdepth: isize,
}

impl WordBranch{
    /// selfが数字か、または、それ以外なのかを判定する関数
    /// 数字のときはtrueを返却します
    pub fn self_is_num(&self) -> Result<bool, GenerateError>{
        // あとで16進数表記、8進数表記、2進数表記
        // この関数の実装は一時的なものであとで、詳細な実装が必要になる。
        for (i, j    ) in self.contents.chars() .enumerate(){
            if j == '.' {
                // 数字以外だった場合
                if i == 0 || i == self.contents.len() - 1 {
                    // 不正な数値表現
                    return Err(GenerateError::InvalidNum);
                } else {
                    // pass
                }
            } else if !j.is_ascii_digit() {
                 return Ok(false);
            } else {
                 // 数字だった場合はpass
            }
        }
        Ok(true)
    }
}


impl ASTBranch for WordBranch {
    fn show(&self) {
        println!(
            "{}Word \"{}\"",
            " ".repeat(self.depth as usize * 4),
            self.contents
        )
    }

    fn get_show_as_string(&self) -> String {
        format!(
            "{}Word \"{}\"",
            " ".repeat(self.depth as usize * 4),
            self.contents
        )
    }
}
