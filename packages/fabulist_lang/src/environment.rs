use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::ast::expr::Expr;

pub struct Environment {
    map: HashMap<String, Expr>,
    parent: Option<Weak<RefCell<Environment>>>,
    child: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Self {
            map: HashMap::new(),
            parent: None,
            child: None,
        }))
    }
    pub fn map(&self) -> &HashMap<String, Expr> {
        &self.map
    }
    pub fn mut_map(&mut self) -> &mut HashMap<String, Expr> {
        &mut self.map
    }
    pub fn parent(&self) -> Option<&Weak<RefCell<Environment>>> {
        self.parent.as_ref()
    }
    pub fn child(&self) -> Option<&Rc<RefCell<Environment>>> {
        self.child.as_ref()
    }
    pub fn value(&self, key: impl Into<String>) -> Option<Expr> {
        let key = key.into();
        if let Some(value) = self.map.get(&key) {
            Some(value.clone())
        } else {
            if let Some(parent) = self.parent.as_ref() {
                if let Some(rc_parent) = parent.upgrade() {
                    return rc_parent.deref().borrow().value(key);
                }
            }
            None
        }
    }
    pub fn set_parent(&mut self, environment: Weak<RefCell<Environment>>) {
        self.parent = Some(environment);
    }
    pub fn nest_child(parent: &Rc<RefCell<Environment>>, environment: &Rc<RefCell<Environment>>) {
        environment
            .deref()
            .borrow_mut()
            .set_parent(Rc::downgrade(parent));
        parent.deref().borrow_mut().child = Some(environment.clone());
    }
    pub fn unwrap<'a>(environment: &'a Rc<RefCell<Environment>>) -> Ref<'a, Environment> {
        environment.deref().borrow()
    }
    pub fn unwrap_mut<'a>(environment: &'a Rc<RefCell<Environment>>) -> RefMut<'a, Environment> {
        environment.deref().borrow_mut()
    }
    pub fn insert(environment: &Rc<RefCell<Environment>>, key: impl Into<String>, value: Expr) {
        Environment::unwrap_mut(environment)
            .mut_map()
            .insert(key.into(), value);
    }
    pub fn get_value(
        environment: &Rc<RefCell<Environment>>,
        key: impl Into<String>,
    ) -> Option<Expr> {
        Environment::unwrap(environment).value(key)
    }
}

#[cfg(test)]
mod environment_tests {
    use crate::ast::expr::primary::PrimaryExpr;

    use super::*;

    #[test]
    fn nests_child() {
        let environment = Environment::new();
        let child = Environment::new();
        Environment::nest_child(&environment, &child);

        let nested_child = Environment::unwrap(&environment)
            .child()
            .expect("Environment does not have a child")
            .clone();

        assert!(Rc::ptr_eq(&child, &nested_child))
    }

    #[test]
    fn propagates_value() {
        let environment = Environment::new();
        Environment::insert(&environment, "number", PrimaryExpr::Number(5).into());

        let child = Environment::new();
        Environment::nest_child(&environment, &child);

        if let Some(expr) = Environment::get_value(&child, "number") {
            if let Expr::Primary(primary) = expr {
                if let PrimaryExpr::Number(num) = *primary {
                    assert_eq!(num, 5);
                }
            }
        }
    }
}
