use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use super::value::Value;

#[derive(Debug, Default)]
struct Frame {
    values: BTreeMap<String, Value>,
    parent: Option<Scope>,
}

#[derive(Clone, Debug, Default)]
pub struct Scope(Rc<RefCell<Frame>>);

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(&self) -> Self {
        Self(Rc::new(RefCell::new(Frame {
            values: BTreeMap::new(),
            parent: Some(self.clone()),
        })))
    }

    pub fn define(&self, name: impl Into<String>, value: Value) {
        self.0.borrow_mut().values.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let frame = self.0.borrow();
        if let Some(value) = frame.values.get(name) {
            Some(value.clone())
        } else {
            frame.parent.as_ref().and_then(|parent| parent.get(name))
        }
    }

    pub fn assign(&self, name: &str, value: Value) -> bool {
        let mut frame = self.0.borrow_mut();
        if frame.values.contains_key(name) {
            frame.values.insert(name.to_string(), value);
            true
        } else if let Some(parent) = frame.parent.as_ref() {
            parent.assign(name, value)
        } else {
            false
        }
    }
}
