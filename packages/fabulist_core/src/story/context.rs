use std::collections::HashMap;

#[derive(Debug)]
pub enum ContextValue {
    Integer(i32),
    Bool(bool),
    String(String),
}

#[derive(Debug)]
pub struct Context(pub HashMap<String, ContextValue>);

impl Context {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
}
