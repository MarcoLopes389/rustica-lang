use crate::frontend::{ast::{Program, Stmt, StmtType}, parser::Parser};

#[derive(Debug)]
pub enum ResultType {
    Number,
    Null
}

#[derive(Debug)]
pub struct Result {
    pub value: String,
    kind: ResultType
}

pub struct Interpreter {
    ast: Program
}

impl Interpreter {
    pub fn new(source: &str) -> Interpreter {
        let mut parser = Parser::new(source);
        let ast = parser.produce_ast();
        Interpreter {
            ast
        }
    }

    pub fn evaluate(&mut self) -> Result {
        let mut last_result: Result = Result { value: "".to_string(), kind: ResultType::Null };
        for stmt in &self.ast.body {
            last_result = self.evaluate_stmt(Clone::clone(stmt))
        }

        return last_result
    }

    pub fn evaluate_stmt(&self, stmt: Stmt) -> Result {
        match stmt.kind {
            StmtType::Program => todo!(),
            StmtType::BinaryOp => self.eval_math(stmt),
            StmtType::NumericValue => Result { value: stmt.value.unwrap(), kind: ResultType::Number },
        }
    }

    pub fn eval_math(&self, stmt: Stmt) -> Result {
        let left = self.evaluate_stmt(stmt.left.unwrap());
        let right = self.evaluate_stmt(stmt.right.unwrap());

        let operator = stmt.operator.unwrap();

        if operator == "*" {
            return Result {
                kind: ResultType::Number,
                value: (left.value.parse::<f64>().unwrap() * right.value.parse::<f64>().unwrap()).to_string()
            }
        } else if operator == "-" {
            return Result {
                kind: ResultType::Number,
                value: (left.value.parse::<f64>().unwrap() - right.value.parse::<f64>().unwrap()).to_string()
            }
        } else if operator == "+" {
            return Result {
                kind: ResultType::Number,
                value: (left.value.parse::<f64>().unwrap() - right.value.parse::<f64>().unwrap()).to_string()
            }
        } else if operator == "%" {
            return Result {
                kind: ResultType::Number,
                value: (left.value.parse::<f64>().unwrap() - right.value.parse::<f64>().unwrap()).to_string()
            }
        } else {
            panic!("Operator not available for interpretation")
        }
    }
}