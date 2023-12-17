use crate::frontend::ast::{Stmt, StmtType};

use super::{lexer::{Token, Lexer, TokenType}, ast::Program};
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn not_eof(&self) -> bool {
        match self.tokens.get(0) {
            Some(_) => true,
            None => false
        }
    }

    pub fn first(&self) -> Token {
        match self.tokens.get(0) {
            Some(token) => Clone::clone(token),
            None => Token { value: "".to_string(), kind: TokenType::Null}
        }
    }

    pub fn rem(&mut self) -> Token {
        return self.tokens.remove(0);
    }

    pub fn new(source: &str) -> Parser {
        let tokens = Lexer::tokenize(source);
        Parser {
            tokens,
        }
    }

    pub fn produce_ast(&mut self) -> Program {
        let mut program = Program {
            kind: StmtType::Program,
            body: vec![]
        };

        while self.not_eof() {
            let stmt = self.parse_stmt();
            program.body.push(stmt)
        }

        return program
    }

    pub fn parse_addition_stmt(&mut self) -> Stmt {
        let mut left = self.parse_multi_expr();

        while self.first().value == "+" || self.first().value == "-" {
            let operator = self.rem();
            let right = self.parse_multi_expr();
            left = Stmt {
                kind: StmtType::BinaryOp,
                left: Box::new(Some(left)),
                operator: Some(operator.value),
                right: Box::new(Some(right)),
                value: None
            }
        }
        return left
    }

    pub fn parse_multi_expr(&mut self) -> Stmt {
        let mut left = self.parse_primary_expr();

        while self.first().value == "*" || self.first().value == "/" {
            let operator = self.rem();
            let right = self.parse_primary_expr();
            left = Stmt {
                kind: StmtType::BinaryOp,
                left: Box::new(Some(left)),
                operator: Some(operator.value),
                right: Box::new(Some(right)),
                value: None
            }
        }
        return left
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        return self.parse_addition_stmt()
    }

    pub fn parse_primary_expr(&mut self) -> Stmt {
        let token = self.first();

        match token.kind {
            TokenType::Number => {
                self.rem();
                return Stmt {
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
}