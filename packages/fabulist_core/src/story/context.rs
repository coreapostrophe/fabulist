use std::collections::HashMap;

#[derive(Debug)]
pub enum ContextValue {
    Integer(i32),
    Bool(bool),
    String(String),
}

impl From<&str> for ContextValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for ContextValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i32> for ContextValue {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<bool> for ContextValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[derive(Debug)]
pub struct Context(HashMap<String, ContextValue>);

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn value(&self) -> &HashMap<String, ContextValue> {
        &self.0
    }
    pub fn mut_value(&mut self) -> &mut HashMap<String, ContextValue> {
        &mut self.0
    }
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<ContextValue>) {
        self.0.insert(key.into(), value.into());
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
}
