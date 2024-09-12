use crate::models::values::Value;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ExitCode {
    ExitError = 65,
    RuntimeError = 70,
}

#[derive(Debug)]
pub enum ValueError {
    Error(String),
    Return(Value),
}

impl Display for ValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ValueError {}
