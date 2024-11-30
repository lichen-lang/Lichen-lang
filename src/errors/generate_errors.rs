/// コードの生成時に発生したエラーについて出力します
#[derive(Debug)]
pub enum GenerateError {
    InvalidNum,     // 不正な数字表現があった場合に発生します
    InvalidOperation, // 非対応の演算子を使った場合
    InvalidleftPattern,
    InvalidTypeError,
    // developer向けのエラーです。
    Deverror,
}
