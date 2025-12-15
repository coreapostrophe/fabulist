use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
};

#[derive(Debug)]
pub enum ContextValue {
    Integer(f32),
    Bool(bool),
    String(String),
    None,
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

impl From<f32> for ContextValue {
    fn from(value: f32) -> Self {
        Self::Integer(value)
    }
}

impl From<bool> for ContextValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

pub trait Contextual: Debug {
    fn insert(&mut self, key: String, value: ContextValue);
    fn get(&self, key: &str) -> Option<&ContextValue>;
    fn assign(&mut self, key: String, new_value: ContextValue) -> bool;
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

    pub fn insert(&mut self, key: String, value: ContextValue) {
        self.0.insert(key, value);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Contextual for Context {
    fn insert(&mut self, key: String, value: ContextValue) {
        self.0.insert(key, value);
    }
    fn get(&self, key: &str) -> Option<&ContextValue> {
        self.0.get(key)
    }
    fn assign(&mut self, key: String, new_value: ContextValue) -> bool {
        match self.0.entry(key) {
            Entry::Occupied(mut entry) => {
                entry.insert(new_value);
                true
            }
            Entry::Vacant(_) => false,
        }
    }
}
