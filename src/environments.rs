use crate::models::values::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }
}
