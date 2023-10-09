use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use crate::util::hashmap_to_str;

#[derive(Debug, PartialEq)]
pub enum ContextValue {
    Integer(i32),
    Bool(bool),
}

impl Display for ContextValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextValue::Integer(integer) => write!(f, "{}", integer),
            ContextValue::Bool(boolean) => write!(f, "{}", boolean)
        }
    }
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

impl Display for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context {{{}}}", hashmap_to_str(&self.0))
    }
}

#[cfg(test)]
mod context_tests {
    use super::*;

    #[test]
    fn it_displays() {
        let mut context = Context::new();
        context.0.insert(String::from("context-1"), ContextValue::Bool(false));
        assert_eq!(
            context.to_string(),
            "Context {{key: context-1, value: false}}"
        );
    }
}
