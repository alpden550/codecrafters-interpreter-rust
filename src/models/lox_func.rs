use crate::environments::Environment;
use crate::errors::ValueError;
use crate::interpreter::Interpreter;
use crate::models::callable::Callable;
use crate::models::statements::Stmt;
use crate::models::tokens::Token;
use crate::models::values::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct LoxFunction {
    token: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl LoxFunction {
    pub fn new(token: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        LoxFunction {
            token,
            params,
            body,
        }
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "func")
    }
}

impl Callable for LoxFunction {
    fn to_string(&self) -> &str {
        self.token.name.as_str()
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, ValueError> {
        let mut env = Environment::new(Some(Box::new(interpreter.environment.clone()))); // globals
        for (i, param) in self.params.iter().enumerate() {
            env.define(param.name.clone(), args.get(i).unwrap().clone());
        }

        match interpreter.execute_block(&self.body, env) {
            Ok(_) => {}
            Err(v) => match v {
                ValueError::Error(_) => {}
                ValueError::Return(v) => return Ok(v),
            },
        };

        Ok(Value::Nil)
    }
}
