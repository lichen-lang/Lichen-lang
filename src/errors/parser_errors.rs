#[derive(Debug)]
pub enum ParserError {
    QuotationNotClosed,
    BraceNotOpened,
    BraceNotClosed,
    GroupingSyntaxBoxError, // please write \"if\",\"while\" or \"for\" statement head
    OperationError,         // OperationError 見つからない場合
    CommentBlockNotClosed,
    // for developers
    UnexpectedType,
    DevError,
    Uncategorized,
}
