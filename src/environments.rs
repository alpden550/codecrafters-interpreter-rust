use crate::models::values::Value;
use std::cell::RefCell;

use crate::models::tokens::Token;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
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
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, token: &Token, value: Value) -> Result<Value, String> {
        if self.values.contains_key(&token.name) {
            self.values.insert(token.name.clone(), value.clone());
            Ok(value)
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(token, value)
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
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(token)
        } else {
            Err(format!(
                "[line {}] Undefined variable '{}'.",
                token.line_number, token.name
            ))
        }
    }
}
