use crate::errors::generate_errors::GenerateError;


enum wasm_instructions
{
    // 
    local_get,
    local_set,
    // i32 
    const_i32,
    add_i32,
    // i64
    const_i64,
    add_i64,
    // f32
    const_f32,
    // f64
    const_f64,
}



/// wasm target support
///
/// この関数では関数の呼びだしに対して
/// wasmランタイムの命令に従ったwat形式を返却します。
/// ここでの変換則がlichenのすべてのルールに対応でき
/// ているわけではないことに注意してください
///
/// このトレイトはブランチ構造体に実装します。
///
pub trait Wasm_gen {

    fn generate_wasm(&self) -> Result<String, GenerateError>;
}


