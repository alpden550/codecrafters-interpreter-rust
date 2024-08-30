use crate::models::values::Value;

use crate::models::tokens::Token;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Value>,
}

impl<'a> Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.values {
            writeln!(f, "{key}={value}")?;
        }
        Ok(())
    }
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, token: &Token, value: Value) -> Result<Value, String> {
        if self.values.contains_key(token.name.as_str()) {
            self.values.insert(token.name.clone(), value.clone());
            Ok(value)
        } else if let Some(ref mut enclosing) = self.enclosing {
            enclosing.assign(token, value)
        } else {
            Err(format!(
                "[line {}] Undefined variable '{}'.",
                token.line_number, token.name
            ))
        }
    }

    pub fn get(&self, token: &Token) -> Result<Value, String> {
        if let Some(value) = self.values.get(token.name.as_str()) {
            Ok(value.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.get(token)
        } else {
            Err(format!(
                "[line {}] Undefined variable '{}'.",
                token.line_number, token.name
            ))
        }
    }
}
