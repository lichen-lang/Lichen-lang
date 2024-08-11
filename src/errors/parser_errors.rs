pub enum ParserError {
    QuotationNotClosed,
    BraceNotClosed,
    GroupingSyntaxBoxError, // please write \"if\",\"while\" or \"for\" statement head
    // for developers
    Uncategorized,
}
