#[derive(Debug, Clone)]
pub enum StmtType {
    Program,
    BinaryOp,
    NumericValue,
}

#[derive(Debug)]
pub struct Program {
    pub kind: StmtType,
    pub body: Vec<Stmt>
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtType,
    pub left: Box<Option<Stmt>>,
    pub right: Box<Option<Stmt>>,
    pub value: Option<String>,
    pub operator: Option<String>
}
