#[derive(Debug, Clone, PartialEq)]
pub enum StmtType {
    Program,
    NumericLiteral,
    BinaryExpr,
    Identifier,
    BlockStmt,
    IfStmt,
}

#[derive(Debug)]
pub struct Program {
    pub kind: StmtType,
    pub body: Vec<Stmt>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub kind: StmtType,
    pub left: Option<Box<Stmt>>,
    pub right: Option<Box<Stmt>>,
    pub value: Option<String>,
    pub body: Option<Vec<Stmt>>,
    pub consequent: Option<Box<Stmt>>,
    pub operator: Option<String>
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Equals,
    EqualsEquals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Comma,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Binary,
    Number,
    Identifier,
    Eof,
    Dot,
    Null,
    String,
    If,
    Else,
    Until,
    Unless,
    While,
    Work,
    Interop
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub value: String
}
