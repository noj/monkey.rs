#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Illegal(char),
    Eof,

    // Identifiers + literals
    Ident(&'a str),
    Int(i64),
    String(&'a str),

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
    NotEq,

    // Delimiters
    Comma,
    Semicolon,
    Colon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
