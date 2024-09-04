use crate::models::values::Value;

#[derive(Debug)]
pub enum ExitCode {
    ExitError = 65,
    RuntimeError = 70,
}

pub enum ValueError {
    Error(String),
    Return(Value),
}
