use std::collections::HashMap;

pub enum ContextValue {
    Integer(i32),
    Bool(bool),
}

pub struct Context(pub HashMap<String, ContextValue>);

impl Context {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn insert(&mut self, key: String, value: ContextValue) {
        self.0.insert(key, value);
    }
    pub fn get(&self, key: &str) -> Option<&ContextValue> {
        self.0.get(key)
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut ContextValue> {
        self.0.get_mut(key)
    }
}
