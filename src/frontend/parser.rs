use crate::frontend::ast::{Stmt, StmtType};

use super::{lexer::{Token, TokenType}, ast::Program};

fn not_eof(tokens: &Vec<Token>) -> bool {
    match tokens.get(0) {
        Some(_) => true,
        None => false
    }
}

fn first(tokens: &Vec<Token>) -> Token {
    match tokens.get(0) {
        Some(token) => Clone::clone(token),
        None => Token { value: "".to_string(), kind: TokenType::Null}
    }
}

fn rem(tokens: &mut Vec<Token>) -> Token { tokens.remove(0) }

pub fn produce_ast(tokens: &mut Vec<Token>) -> Program {
    let mut program = Program {
        kind: StmtType::Program,
        body: vec![]
    };

    while not_eof(tokens) {
        let stmt = parse_stmt(tokens);
        program.body.push(stmt)
    }

    program
}

fn parse_addition_stmt(tokens: &mut Vec<Token>) -> Stmt {
    let mut left = parse_multi_expr(tokens);

    while first(tokens).value == "+" || first(tokens).value == "-" {
        let operator = rem(tokens);
        let right = parse_multi_expr(tokens);
        left = Stmt {
            kind: StmtType::BinaryOp,
            left: Box::new(Some(left)),
            operator: Some(operator.value),
            right: Box::new(Some(right)),
            value: None
        }
    }
    left
}

fn parse_multi_expr(tokens: &mut Vec<Token>) -> Stmt {
    let mut left = parse_primary_expr(tokens);

    while first(tokens).value == "*" || first(tokens).value == "/" {
        let operator = rem(tokens);
        let right = parse_primary_expr(tokens);
        left = Stmt {
            kind: StmtType::BinaryOp,
            left: Box::new(Some(left)),
            operator: Some(operator.value),
            right: Box::new(Some(right)),
            value: None
        }
    }
    left
}

fn parse_stmt(tokens: &mut Vec<Token>) -> Stmt {
    parse_addition_stmt(tokens)
}

fn parse_primary_expr(tokens: &mut Vec<Token>) -> Stmt {
    let token = first(tokens);

    match token.kind {
        TokenType::Number => {
            rem(tokens);
            Stmt {
                kind: StmtType::NumericValue,
                left: Box::new(None),
                right: Box::new(None),
                value: Some(token.value),
                operator: None,
            }
        },
        TokenType::Identifier => todo!(),
        _ => panic!("Unespected token on parse")
    }
}
