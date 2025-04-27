use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    Ident(String),
    Int(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    Neq,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal => f.write_str("ILLEGAL"),
            Token::EOF => f.write_str("EOF"),

            Token::Ident(ident) => f.write_str(ident),
            Token::Int(int) => f.write_str(int),

            Token::Assign => f.write_str("="),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Bang => f.write_str("!"),
            Token::Asterisk => f.write_str("*"),
            Token::Slash => f.write_str("/"),

            Token::Lt => f.write_str("<"),
            Token::Gt => f.write_str(">"),
            Token::Eq => f.write_str("=="),
            Token::Neq => f.write_str("!="),

            Token::Comma => f.write_str(","),
            Token::Semicolon => f.write_str(";"),

            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),

            Token::Function => f.write_str("fn"),
            Token::Let => f.write_str("let"),
            Token::True => f.write_str("true"),
            Token::False => f.write_str("false"),
            Token::If => f.write_str("if"),
            Token::Else => f.write_str("else"),
            Token::Return => f.write_str("return"),
        }
    }
}
