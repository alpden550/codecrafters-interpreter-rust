use crate::errors::ValueError;
use crate::interpreter::Interpreter;
use crate::models::values::Value;
use std::fmt::Debug;

#[allow(dead_code)]
pub trait Callable: Debug {
    fn to_string(&self) -> &str;
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, ValueError>;
}
