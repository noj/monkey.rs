mod lexer;

fn main() {
    let tok = lexer::Token::Let;
    println!("{:?}", tok);
}
