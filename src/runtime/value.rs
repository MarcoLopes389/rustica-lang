use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Null,
    String(String),
    Identifier(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Identifier(id) => write!(f, "{}", id),
        }
    }
}