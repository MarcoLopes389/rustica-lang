use std::fmt;
use crate::frontend::parser::ParserError;
use crate::runtime::value::Value;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RuntimeError {
    ParserError(ParserError),
    TypeError { message: String },
    UnknownOperator { operator: String },
    DivisionByZero,
    UnrecognizedValueType { expected: String, found: Value },
    UndefinedVariable { name: String },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::ParserError(e) => write!(f, "Erro de Parsing: {}", e),
            RuntimeError::TypeError { message } => write!(f, "Erro de Tipo: {}", message),
            RuntimeError::UnknownOperator { operator } => write!(f, "Operador desconhecido: '{}'", operator),
            RuntimeError::DivisionByZero => write!(f, "Erro de Execução: Divisão por zero."),
            RuntimeError::UnrecognizedValueType { expected, found } => {
                write!(f, "Erro de Execução: Tipo de valor inesperado. Esperava {}, mas encontrou {:?}", expected, found)
            },
            RuntimeError::UndefinedVariable { name } => write!(f, "Erro de Execução: Variável não definida '{}'", name),
        }
    }
}

impl From<ParserError> for RuntimeError {
    fn from(error: ParserError) -> Self {
        RuntimeError::ParserError(error)
    }
}