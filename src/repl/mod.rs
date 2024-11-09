use std::io::{self, BufRead, Write};

use crate::lexer::{new_lexer, Token};

pub fn start() {
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    loop {
        stdout.write(">> ".as_bytes()).unwrap();
        stdout.flush().unwrap();

        let mut line = String::new();
        if let Ok(n) = stdin.lock().read_line(&mut line) {
            if n == 0 {
                // EOF
                return;
            }
        }

        let mut lex = new_lexer(&line);
        loop {
            let tok = lex.next_token();
            println!("{:?}", tok);

            if tok == Token::Eof {
                break;
            }
        }
    }
}
