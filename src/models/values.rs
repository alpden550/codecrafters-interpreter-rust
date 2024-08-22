use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    String(String),
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}

impl Value {
    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(s) => s.clone().into(),
            _ => None,
        }
    }

    pub fn get_number(&self) -> Option<f64> {
        match self {
            Self::Number(f) => (*f).into(),
            _ => None,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Bool(b) => *b == true,
            Self::String(s) => !s.is_empty(),
            Self::Number(n) => *n != 0.0,
        }
    }

    pub fn is_equal(&self, other: Self) -> bool {
        match (self, other) {
            (Self::Nil, Value::Nil) => true,
            (Self::Bool(l), Value::Bool(r)) => *l == r,
            (Self::Number(l), Value::Number(r)) => *l == r,
            (Self::String(l), Value::String(r)) => *l == r,
            _ => false,
        }
    }
}
