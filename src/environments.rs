use crate::parsers::expressions::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Environment {
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
            values: HashMap::new(),
        }
    }

    pub fn insert_number(&mut self, key: String, value: f64) {
        self.values.insert(key, Value::Number(value));
    }

    pub fn insert_text(&mut self, key: String, value: String) {
        self.values.insert(key, Value::String(value));
    }

    pub fn insert_bool(&mut self, key: String, value: bool) {
        self.values.insert(key, Value::Bool(value));
    }

    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }
}
