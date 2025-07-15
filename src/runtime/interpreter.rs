use crate::frontend::{ast::{Stmt, StmtType}};
use crate::frontend::ast::Program;
use crate::frontend::lexer::tokenize;
use crate::frontend::parser::produce_ast;

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

pub fn interpret(source_code: &str) -> Result {
    let mut tokens = tokenize(source_code);
    let ast = produce_ast(&mut tokens);
    evaluate(ast)
}

fn evaluate(ast: Program) -> Result {
    let mut last_result: Result = Result { value: "".to_string(), kind: ResultType::Null };
    for stmt in ast.body {
        last_result = evaluate_stmt(stmt)
    }

    last_result
}

fn evaluate_stmt(stmt: Stmt) -> Result {
    match stmt.kind {
        StmtType::Program => todo!(),
        StmtType::BinaryOp => eval_math(stmt),
        StmtType::NumericValue => Result { value: stmt.value.unwrap(), kind: ResultType::Number },
    }
}

fn eval_math(stmt: Stmt) -> Result {
    let left = evaluate_stmt(stmt.left.unwrap());
    let right = evaluate_stmt(stmt.right.unwrap());

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
            value: (left.value.parse::<f64>().unwrap() + right.value.parse::<f64>().unwrap()).to_string()
        }
    } else if operator == "%" {
        return Result {
            kind: ResultType::Number,
            value: (left.value.parse::<f64>().unwrap() % right.value.parse::<f64>().unwrap()).to_string()
        }
    } else {
        panic!("Operator not available for interpretation")
    }
}