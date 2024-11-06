pub mod token;
pub use token::*;

pub struct Lexer {
    input: String,
    pos: usize,
    read_pos: usize,
    ch: u8,
}

pub fn new(input: String) -> Lexer {
    let mut res = Lexer {
        input,
        pos: 0,
        read_pos: 0,
        ch: 0,
    };

    res.read_char();
    res
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

impl Lexer {
    /// Reads next character in input or stops
    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_pos];
        }

        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_pos >= self.input.len() {
            return '\0';
        } else {
            return self.input.as_bytes()[self.read_pos] as char;
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch as char {
                ' ' | '\t' | '\n' | '\r' => self.read_char(),
                _ => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch as char {
            '"' => Token::String(self.read_string()),

            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }

            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }

            '(' => Token::Lparen,
            ')' => Token::Rparen,

            '{' => Token::Lbrace,
            '}' => Token::Rbrace,

            '[' => Token::Lbracket,
            ']' => Token::Rbracket,

            ':' => Token::Colon,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::Lt,
            '>' => Token::Gt,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,

            '\0' => Token::Eof,

            ch => {
                if is_letter(ch) {
                    let literal = self.read_identifier();
                    return match literal.as_str() {
                        "fn" => Token::Function,
                        "let" => Token::Let,
                        "true" => Token::True,
                        "false" => Token::False,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,

                        _ => Token::Ident(literal),
                    };
                } else if is_digit(ch) {
                    let literal = self.read_number();
                    return Token::Int(literal);
                } else {
                    Token::Illegal(ch)
                }
            }
        };

        self.read_char();
        tok
    }

    pub fn read_identifier(&mut self) -> String {
        let pos = self.pos;
        while is_letter(self.ch as char) {
            self.read_char();
        }

        let lit = &self.input[pos..self.pos];
        lit.into()
    }

    pub fn read_number(&mut self) -> i64 {
        let pos = self.pos;
        while is_digit(self.ch as char) {
            self.read_char();
            println!("ch={}", self.ch as char);
        }

        let lit = &self.input[pos..self.pos];
        // Can't fail:
        lit.parse::<i64>().unwrap()
    }

    pub fn read_string(&mut self) -> String {
        let pos = self.pos + 1;
        loop {
            self.read_char();

            if self.ch as char == '"' || self.ch as char == '\0' {
                break;
            }
        }

        let slice = &self.input[pos..self.pos];
        slice.into()
    }
}

#[test]
fn test_next_token() {
    let input = r"let fifteen = 15;
        let add = fn(x, y) {
            x + y
        };

        "
    .into();

    let expects = vec![
        // Statement 1
        Token::Let,
        Token::Ident("fifteen".into()),
        Token::Assign,
        Token::Int(15),
        Token::Semicolon,
        // Statement 2
        Token::Let,
        Token::Ident("add".into()),
        Token::Assign,
        Token::Function,
        Token::Lparen,
        Token::Ident("x".into()),
        Token::Comma,
        Token::Ident("y".into()),
        Token::Rparen,
        Token::Lbrace,
        Token::Ident("x".into()),
        Token::Plus,
        Token::Ident("y".into()),
        Token::Rbrace,
        Token::Semicolon,
        Token::Eof,
    ];

    let mut lexer = new(input);
    for expected in expects {
        let tok = lexer.next_token();
        println!("{:?}", tok);
        assert_eq!(expected, tok);
    }
}
