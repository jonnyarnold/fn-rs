#[derive(Debug)]
pub enum Token {
    // Valued Tokens
    Symbol(String),
    Number(i64),
    String(String),

    // Operators
    // (TODO: Replace with a generic Operator token?)
    Plus,
    Minus,
    Star,
    Slash,

    // Brackets
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
}
