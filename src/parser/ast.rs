enum Statement {
    Let,
    Return,
    Expression(Expression),
    Block(Vec<Statement>),
}

enum Expression {
    Identifier,
    Int,
    Bool,
    String,
    Prefix,
    Infix,
    If,
    Fun,
    Call,
    Array,
    Index,
    Hash,
}

struct Program {
    statements: Vec<Statement>,
}
