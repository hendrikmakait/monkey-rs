#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EoF,

    Ident(String),
    Int(i64),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
}
