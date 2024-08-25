use crate::models::values::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Value>,
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.values {
            writeln!(f, "{key}={value}")?;
        }
        Ok(())
    }
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        if let Some(ref mut e) = self.enclosing {
            e.insert(key, value);
            return;
        }
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        if let Some(ref e) = self.enclosing {
            match e.get(key) {
                Some(v) => Some(v),
                None => self.values.get(key),
            }
        } else {
            self.values.get(key)
        }
    }
}
