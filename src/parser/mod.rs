pub mod ast;
pub use ast::*;

use crate::lexer::{new_lexer, Lexer, Token};

enum ParseError {
    IllegalChar(char),
    SyntaxError,
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    errors: Vec<String>,

    cur_token: Token<'a>,
    peek_token: Token<'a>,
    // FIXME: prefix and postfix parse fn
}

pub fn new_parser<'a>(input: &'a str) -> Parser<'a> {
    Parser {
        lexer: new_lexer(input),
        errors: Vec::new(),

        cur_token: Token::Eof,
        peek_token: Token::Eof,
    }
}

fn precedence(token: Token) -> Option<i32> {
    match token {
        Token::Eq => Some(1),
        Token::NotEq => Some(1),

        Token::Lt => Some(2),
        Token::Gt => Some(2),

        Token::Plus => Some(3),
        Token::Minus => Some(3),

        Token::Slash => Some(4),
        Token::Asterisk => Some(4),

        Token::Lparen => Some(5),

        Token::Lbracket => Some(6),

        _ => None,
    }
}

impl<'a> Parser<'a> {
    pub fn parse_program() -> Result<(), ParseError> {
        Result::Ok(())
    }
}
