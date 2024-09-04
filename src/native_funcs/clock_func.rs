use crate::errors::ValueError;
use crate::interpreter::Interpreter;
use crate::models::callable::Callable;
use crate::models::values::Value;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct ClockFunction;

impl Callable for ClockFunction {
    fn to_string(&self) -> &str {
        "clock"
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: &[Value],
    ) -> Result<Value, ValueError> {
        // Return the current time in seconds since the epoch
        let start = SystemTime::now();
        match start.duration_since(UNIX_EPOCH) {
            Ok(duration) => Ok(Value::Number(duration.as_secs_f64())),
            Err(_) => Err(ValueError::Error(
                "System time before UNIX epoch!".to_string(),
            )),
        }
    }
}

impl ClockFunction {
    pub fn to_string(&self) -> String {
        String::from("clock")
    }
}
