//! Runtime environment shared by the interpreter.
//!
//! Values are stored in nested environments backed by [`Rc`] + [`RefCell`], allowing
//! lightweight scoping for expressions and statements.
//!
//! [`Rc`]: std::rc::Rc
//! [`RefCell`]: std::cell::RefCell
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{hash_map::Entry, HashMap},
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::{
    error::{OwnedSpan, RuntimeError},
    interpreter::runtime_value::RuntimeValue,
};

/// Shared pointer to an [`Environment`], used throughout evaluation.
///
/// Cloning the alias keeps references to the same interior state.
pub type RuntimeEnvironment = Rc<RefCell<Environment>>;

/// Runtime environment with optional parent/child links.
#[derive(Debug)]
pub struct Environment {
    map: HashMap<String, RuntimeValue>,
    parent: Option<Weak<RefCell<Environment>>>,
    child: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// Creates an empty environment with no parent or child.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fabulist_lang::interpreter::environment::Environment;
    ///
    /// let env = Environment::new();
    /// assert!(Environment::get_child(&env).is_none());
    /// ```
    pub fn new() -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Self {
            map: HashMap::new(),
            parent: None,
            child: None,
        }))
    }
    fn mut_map(&mut self) -> &mut HashMap<String, RuntimeValue> {
        &mut self.map
    }
    fn child(&self) -> Option<&Rc<RefCell<Environment>>> {
        self.child.as_ref()
    }
    fn value(&self, key: impl Into<String>) -> Option<RuntimeValue> {
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
    fn set_parent(&mut self, environment: Weak<RefCell<Environment>>) {
        self.parent = Some(environment);
    }
    /// Links an existing environment as a child of the given parent.
    pub fn nest_child(parent: &Rc<RefCell<Environment>>, environment: &Rc<RefCell<Environment>>) {
        environment
            .deref()
            .borrow_mut()
            .set_parent(Rc::downgrade(parent));
        parent.deref().borrow_mut().child = Some(environment.clone());
    }

    /// Attaches a new empty child to the given environment and returns it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fabulist_lang::interpreter::environment::Environment;
    ///
    /// let env = Environment::new();
    /// let child = Environment::add_empty_child(&env);
    /// assert!(Environment::get_child(&env).is_some());
    /// assert!(std::rc::Rc::ptr_eq(
    ///     &child,
    ///     &Environment::get_child(&env).unwrap()
    /// ));
    /// ```
    pub fn add_empty_child(environment: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        let child = Environment::new();
        Environment::nest_child(environment, &child);
        child
    }

    /// Borrows the environment immutably.
    pub fn unwrap(environment: &Rc<RefCell<Environment>>) -> Ref<'_, Environment> {
        environment.deref().borrow()
    }

    /// Borrows the environment mutably.
    pub fn unwrap_mut(environment: &Rc<RefCell<Environment>>) -> RefMut<'_, Environment> {
        environment.deref().borrow_mut()
    }

    /// Inserts a value into the current environment without propagating upward.
    pub fn insert(
        environment: &Rc<RefCell<Environment>>,
        key: impl Into<String>,
        value: RuntimeValue,
    ) {
        Environment::unwrap_mut(environment)
            .mut_map()
            .insert(key.into(), value);
    }
    /// Looks up a value in the current environment, falling back to parents.
    pub fn get_value(
        environment: &Rc<RefCell<Environment>>,
        key: impl Into<String>,
    ) -> Option<RuntimeValue> {
        Environment::unwrap(environment).value(key)
    }

    /// Returns the direct child environment if one exists.
    pub fn get_child(environment: &Rc<RefCell<Environment>>) -> Option<Rc<RefCell<Environment>>> {
        Environment::unwrap(environment).child().cloned()
    }

    /// Assigns a value to an existing binding, walking up parent scopes when needed.
    ///
    /// Returns an error when the identifier does not exist in any accessible scope.
    pub fn assign(
        environment: &Rc<RefCell<Environment>>,
        key: impl Into<String>,
        value: RuntimeValue,
    ) -> Result<(), RuntimeError> {
        let key = key.into();
        let mut env_ref = environment.deref().borrow_mut();

        match env_ref.map.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                Ok(())
            }
            Entry::Vacant(_) => {
                if let Some(parent) = env_ref.parent.as_ref() {
                    if let Some(rc_parent) = parent.upgrade() {
                        return Environment::assign(&rc_parent, key, value);
                    }
                }
                Err(RuntimeError::IdentifierDoesNotExist(OwnedSpan::default()))
            }
        }
    }
}

#[cfg(test)]
mod environment_tests {
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
    fn propagates_value_downwards() {
        let environment = Environment::new();
        Environment::insert(
            &environment,
            "number",
            RuntimeValue::Number {
                value: 5.0,
                span: Default::default(),
            },
        );

        let child = Environment::add_empty_child(&environment);

        let value = Environment::get_value(&child, "number")
            .expect("Could not find propagated value from parent environment");

        match value {
            RuntimeValue::Number { value: num, .. } => assert_eq!(num, 5.0),
            _ => panic!("Propagated value has incorrect type"),
        }
    }

    #[test]
    fn assigns_value_upwards() {
        let environment = Environment::new();
        Environment::insert(
            &environment,
            "number",
            RuntimeValue::Number {
                value: 5.0,
                span: Default::default(),
            },
        );
        let child = Environment::add_empty_child(&environment);

        Environment::assign(
            &child,
            "number",
            RuntimeValue::Number {
                value: 10.0,
                span: Default::default(),
            },
        )
        .expect("Failed to assign value upwards");

        let value = Environment::get_value(&environment, "number")
            .expect("Could not find assigned value in parent environment");

        match value {
            RuntimeValue::Number { value: num, .. } => assert_eq!(num, 10.0),
            _ => panic!("Assigned value has incorrect type"),
        }
    }
}
