use crate::frontend::{ast::{Program, Stmt, StmtType}, lexer::tokenize, parser::{produce_ast}};
use crate::runtime::error::RuntimeError;
use crate::runtime::value::Value;

pub fn interpret(source_code: &str) -> Result<Value, RuntimeError> {
    let mut tokens = tokenize(source_code);

    let ast = produce_ast(&mut tokens)?;

    evaluate(ast)
}

fn evaluate(ast: Program) -> Result<Value, RuntimeError> {
    evaluate_block(ast.body)
}

fn evaluate_block(body: Vec<Stmt>) -> Result<Value, RuntimeError> {
    let mut last_result = Value::Null;
    for stmt in body {
        last_result = evaluate_stmt(stmt)?;
    }
    Ok(last_result)
}


fn evaluate_stmt(stmt: Stmt) -> Result<Value, RuntimeError> {
    match stmt.kind {
        StmtType::Program => {
            Err(RuntimeError::TypeError { message: "Cannot evaluate Program node directly as a statement.".to_string() })
        },
        StmtType::NumericLiteral => {
            let num_str = stmt.value.ok_or_else(|| RuntimeError::TypeError {
                message: "NumericLiteral is missing a value.".to_string()
            })?;
            let num = num_str.parse::<f64>().map_err(|e| RuntimeError::TypeError {
                message: format!("Failed to parse number '{}': {}", num_str, e),
            })?;
            Ok(Value::Number(num))
        },
        StmtType::BinaryExpr => eval_binary_expr(stmt),
        StmtType::Identifier => {
            Err(RuntimeError::UndefinedVariable { name: stmt.value.unwrap_or_default() })
        },
        StmtType::IfStmt => eval_if_stmt(stmt),
        StmtType::BlockStmt => {
            let body = stmt.body.ok_or_else(|| RuntimeError::TypeError {
                message: "BlockStmt is missing its body.".to_string()
            })?;
            evaluate_block(body)
        },
        _ => Err(RuntimeError::TypeError { message: format!("Unhandled AST node type: {:?}", stmt.kind) }),
    }
}

fn eval_binary_expr(stmt: Stmt) -> Result<Value, RuntimeError> {
    let left_ast = stmt.left.ok_or_else(|| RuntimeError::TypeError {
        message: "Binary expression missing left operand.".to_string()
    })?.as_ref().clone();

    let right_ast = stmt.right.ok_or_else(|| RuntimeError::TypeError {
        message: "Binary expression missing right operand.".to_string()
    })?.as_ref().clone();

    let operator = stmt.operator.ok_or_else(|| RuntimeError::TypeError {
        message: "Binary expression missing operator.".to_string()
    })?;

    let left_val = evaluate_stmt(left_ast)?;
    let right_val = evaluate_stmt(right_ast)?;

    match operator.as_str() {
        "*" | "/" | "-" | "+" | "%" => {
            let left_num = match left_val {
                Value::Number(n) => n,
                _ => return Err(RuntimeError::TypeError { message: format!("Left operand of '{}' must be a number, got {}.", operator, left_val) }),
            };
            let right_num = match right_val {
                Value::Number(n) => n,
                _ => return Err(RuntimeError::TypeError { message: format!("Right operand of '{}' must be a number, got {}.", operator, right_val) }),
            };

            let result = match operator.as_str() {
                "*" => left_num * right_num,
                "/" => {
                    if right_num == 0.0 {
                        return Err(RuntimeError::DivisionByZero);
                    }
                    left_num / right_num
                },
                "-" => left_num - right_num,
                "+" => left_num + right_num,
                "%" => left_num % right_num,
                _ => unreachable!(),
            };
            Ok(Value::Number(result))
        },
        "==" | "!=" | "<" | "<=" | ">" | ">=" => {
            let left_num = match left_val {
                Value::Number(n) => n,
                _ => return Err(RuntimeError::TypeError { message: format!("Left operand of comparison '{}' must be a number, got {}.", operator, left_val) }),
            };
            let right_num = match right_val {
                Value::Number(n) => n,
                _ => return Err(RuntimeError::TypeError { message: format!("Right operand of comparison '{}' must be a number, got {}.", operator, right_val) }),
            };

            let result = match operator.as_str() {
                "==" => left_num == right_num,
                "!=" => left_num != right_num,
                "<" => left_num < right_num,
                "<=" => left_num <= right_num,
                ">" => left_num > right_num,
                ">=" => left_num >= right_num,
                _ => unreachable!(),
            };
            Ok(Value::Boolean(result))
        },
        _ => Err(RuntimeError::UnknownOperator { operator }),
    }
}


fn eval_if_stmt(stmt: Stmt) -> Result<Value, RuntimeError> {
    let condition_ast = stmt.left.ok_or_else(|| RuntimeError::TypeError {
        message: "If statement missing condition.".to_string()
    })?.as_ref().clone();

    let consequent_ast = stmt.consequent.ok_or_else(|| RuntimeError::TypeError {
        message: "If statement missing consequent block.".to_string()
    })?.as_ref().clone();

    let condition_val = evaluate_stmt(condition_ast)?;

    let is_truthy = match condition_val {
        Value::Boolean(b) => b,
        Value::Number(n) => n != 0.0,
        Value::Null => false,
        _ => return Err(RuntimeError::TypeError { message: format!("If condition must evaluate to a boolean or number, got {}.", condition_val) }),
    };

    if is_truthy {
        evaluate_stmt(consequent_ast)
    } else if let Some(alternate_box) = stmt.right {
        evaluate_stmt(*alternate_box)
    } else {
        Ok(Value::Null)
    }
}