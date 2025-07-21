use crate::frontend::ast::{Stmt, StmtType};

use super::{ast::{Program, Token, TokenType}};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken { expected: String, found: Token },
    MissingExpression,
    EndOfFileUnexpected,
    Custom(String),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken { expected, found } => {
                write!(f, "Parsing error: unexpected token {:?} ('{:?}') {:?}", expected, found.kind, found.value)
            },
            ParserError::MissingExpression => write!(f, "Parsing error: expected expression not found expression"),
            ParserError::EndOfFileUnexpected => write!(f, "Parsing error: Unexpected end of file"),
            ParserError::Custom(msg) => write!(f, "Parsing error: {}", msg),
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(err: std::io::Error) -> Self {
        ParserError::Custom(format!("I/O Error: {}", err))
    }
}

fn not_eof(tokens: &mut Vec<Token>) -> bool {
    tokens.get(0).map_or(false, |token| token.kind != TokenType::Eof)
}

fn first(tokens: &mut Vec<Token>) -> Result<Token, ParserError> {
    tokens.get(0)
        .cloned()
        .ok_or(ParserError::EndOfFileUnexpected)
}

fn consume(tokens: &mut Vec<Token>) -> Result<Token, ParserError> {
    if tokens.is_empty() {
        return Err(ParserError::EndOfFileUnexpected);
    }
    Ok(tokens.remove(0))
}

fn expect(tokens: &mut Vec<Token>, kind: TokenType) -> Result<Token, ParserError> {
    let token = consume(tokens)?;
    if token.kind != kind {
        return Err(ParserError::UnexpectedToken { expected: format!("{:?}", kind), found: token });
    }
    Ok(token)
}

pub fn produce_ast(tokens: &mut Vec<Token>) -> Result<Program, ParserError> {
    let mut program = Program {
        kind: StmtType::Program,
        body: vec![],
    };

    while not_eof(tokens) {
        let stmt = parse_stmt(tokens)?;
        program.body.push(stmt);
    }

    Ok(program)
}

fn parse_block_stmt(tokens: &mut Vec<Token>) -> Result<Stmt, ParserError> {
    expect(tokens, TokenType::OpenBracket)?;

    let mut body_stmts = Vec::new();
    while not_eof(tokens) && first(tokens)?.kind != TokenType::CloseBracket {
        body_stmts.push(parse_stmt(tokens)?);
    }

    expect(tokens, TokenType::CloseBracket)?;

    Ok(Stmt {
        kind: StmtType::BlockStmt,
        body: Some(body_stmts),
        left: None,
        right: None,
        operator: None,
        value: None,
        consequent: None,
    })
}

fn parse_if_stmt(tokens: &mut Vec<Token>) -> Result<Stmt, ParserError> {
    expect(tokens, TokenType::If)?;
    expect(tokens, TokenType::OpenParen)?;

    let condition = parse_comparison_expr(tokens)?;
    expect(tokens, TokenType::CloseParen)?;

    let consequent = parse_block_stmt(tokens)?;

    let mut alternate: Option<Box<Stmt>> = None;

    if let Ok(else_token) = first(tokens) {
        if else_token.kind == TokenType::Else {
            consume(tokens)?;

            if let Ok(if_token) = first(tokens) {
                if if_token.kind == TokenType::If {
                    alternate = Some(Box::new(parse_if_stmt(tokens)?));
                } else {
                    alternate = Some(Box::new(parse_block_stmt(tokens)?));
                }
            } else {
                return Err(ParserError::MissingExpression);
            }
        }
    }

    Ok(Stmt {
        kind: StmtType::IfStmt,
        left: Some(Box::new(condition)),
        consequent: Some(Box::new(consequent)),
        right: alternate,
        body: None,
        operator: None,
        value: None,
    })
}

fn parse_while_stmt(tokens: &mut Vec<Token>) -> Result<Stmt, ParserError> {
    expect(tokens, TokenType::While)?;
    expect(tokens, TokenType::OpenParen)?;

    let condition = parse_comparison_expr(tokens)?;
    expect(tokens, TokenType::CloseParen)?;

    let consequent = parse_block_stmt(tokens)?;

    Ok(Stmt {
        kind: StmtType::WhileStmt,
        left: Some(Box::new(condition)),
        consequent: Some(Box::new(consequent)),
        right: None,
        body: None,
        operator: None,
        value: None,
    })
}

fn parse_stmt(tokens: &mut Vec<Token>) -> Result<Stmt, ParserError> {
    let current_token_kind = first(tokens)?.kind;

    match current_token_kind {
        TokenType::If => parse_if_stmt(tokens),
        TokenType::While => parse_while_stmt(tokens),
        _ => parse_comparison_expr(tokens),
    }
}

fn parse_comparison_expr(tokens: &mut Vec<Token>) -> Result<Stmt, ParserError> {
    let mut left = parse_addition_expr(tokens)?;

    while let Ok(operator) = first(tokens) {
        let op_kind = &operator.kind;
        if op_kind == &TokenType::EqualsEquals ||
            op_kind == &TokenType::NotEquals ||
            op_kind == &TokenType::LessThan ||
            op_kind == &TokenType::LessThanEquals ||
            op_kind == &TokenType::GreaterThan ||
            op_kind == &TokenType::GreaterThanEquals
        {
            consume(tokens)?;
            let right = parse_addition_expr(tokens)?;
            left = Stmt {
                kind: StmtType::BinaryExpr,
                left: Some(Box::new(left)),
                operator: Some(operator.value),
                right: Some(Box::new(right)),
                value: None,
                consequent: None,
                body: None,
            };
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_addition_expr(tokens: &mut Vec<Token>) -> Result<Stmt, ParserError> {
    let mut left = parse_multiplication_expr(tokens)?;

    while let Ok(operator) = first(tokens) {
        if operator.value == "+" || operator.value == "-" {
            consume(tokens)?;
            let right = parse_multiplication_expr(tokens)?;
            left = Stmt {
                kind: StmtType::BinaryExpr,
                left: Some(Box::new(left)),
                operator: Some(operator.value),
                right: Some(Box::new(right)),
                value: None,
                body: None,
                consequent: None,
            };
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_multiplication_expr(tokens: &mut Vec<Token>) -> Result<Stmt, ParserError> {
    let mut left = parse_primary_expr(tokens)?;

    while let Ok(operator) = first(tokens) {
        if operator.value == "*" || operator.value == "/" || operator.value == "%" {
            consume(tokens)?;
            let right = parse_primary_expr(tokens)?;
            left = Stmt {
                kind: StmtType::BinaryExpr,
                left: Some(Box::new(left)),
                operator: Some(operator.value),
                right: Some(Box::new(right)),
                value: None,
                body: None,
                consequent: None,
            };
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_primary_expr(tokens: &mut Vec<Token>) -> Result<Stmt, ParserError> {
    let token = consume(tokens)?;

    match token.kind {
        TokenType::Number => Ok(Stmt {
            kind: StmtType::NumericLiteral,
            left: None,
            right: None,
            value: Some(token.value),
            operator: None,
            consequent: None,
            body: None,
        }),
        TokenType::Identifier => Ok(Stmt {
            kind: StmtType::Identifier,
            left: None,
            right: None,
            value: Some(token.value),
            operator: None,
            consequent: None,
            body: None,
        }),
        TokenType::OpenParen => {
            let expr = parse_addition_expr(tokens)?;
            expect(tokens, TokenType::CloseParen)?;
            Ok(expr)
        }
        TokenType::Null => Err(ParserError::MissingExpression),
        _ => Err(ParserError::UnexpectedToken {
            expected: "number, identifier, or open parenthesis".to_string(),
            found: token,
        }),
    }
}
