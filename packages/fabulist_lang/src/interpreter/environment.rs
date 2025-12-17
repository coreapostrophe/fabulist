//! Runtime environment stack for interpreter scopes.
//!
//! Environments form a parent/child chain where children hold strong references to their parent
//! via weak pointers to avoid reference cycles. Values are stored in a per-scope map, and lookups
//! traverse parents. Typical usage is to create a root, derive children for nested scopes, and
//! assign or insert values as execution proceeds.
//!
//! # Examples
//!
//! ```rust
//! use fabulist_lang::parser::error::SpanSlice;
//! use fabulist_lang::interpreter::environment::RuntimeEnvironment;
//! use fabulist_lang::interpreter::runtime_value::RuntimeValue;
//!
//! let root = RuntimeEnvironment::new();
//! root.insert_env_value(
//!     "x",
//!     RuntimeValue::Number {
//!         value: 1.0,
//!         span_slice: SpanSlice::default(),
//!     },
//! )
//! .unwrap();
//!
//! let child = root.add_empty_child().unwrap();
//! let RuntimeValue::Number { value, .. } = child.get_env_value("x").unwrap() else {
//!     panic!("expected number");
//! };
//! assert_eq!(value, 1.0);
//!
//! child
//!     .assign_env_value(
//!         "x",
//!         RuntimeValue::Number {
//!             value: 2.0,
//!             span_slice: SpanSlice::default(),
//!         },
//!     )
//!     .unwrap();
//! let RuntimeValue::Number { value, .. } = root.get_env_value("x").unwrap() else {
//!     panic!("expected number");
//! };
//! assert_eq!(value, 2.0);
//! ```

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::interpreter::runtime_value::RuntimeValue;

/// Errors that can occur when manipulating runtime environments.
#[derive(thiserror::Error, Debug)]
pub enum EnvironmentError {
    /// Returned when attempting to access an environment that has already been dropped.
    #[error("RuntimeEnvironment has been dropped")]
    DroppedEnvironment,
    /// Returned when trying to set a strong `RuntimeEnvironment` as a parent (parents must be weak).
    #[error("Cannot set a strong RuntimeEnvironment as parent")]
    StrongParent,
    /// Returned when a requested key is not present in the environment chain.
    #[error("Key `{0}` does not exist in the environment")]
    KeyDoesNotExist(String),
}

#[derive(Debug)]
/// Internal storage for a single scope. Prefer interacting through `RuntimeEnvironment`.
pub struct Environment {
    map: HashMap<String, RuntimeValue>,
    parent: Option<RuntimeEnvironment>,
    child: Option<RuntimeEnvironment>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    /// Create an empty environment with no parent or child.
    pub fn new() -> Environment {
        Self {
            map: HashMap::new(),
            parent: None,
            child: None,
        }
    }

    /// Borrow the internal map for read-only inspection.
    pub fn map(&self) -> &HashMap<String, RuntimeValue> {
        &self.map
    }

    /// Get a reference to the parent `RuntimeEnvironment`, if any.
    pub fn parent(&self) -> Option<&RuntimeEnvironment> {
        self.parent.as_ref()
    }

    /// Get a reference to the child `RuntimeEnvironment`, if any.
    pub fn child(&self) -> Option<&RuntimeEnvironment> {
        self.child.as_ref()
    }

    /// Look up a value in the current environment without traversing parents.
    pub fn get_value(&self, key: impl Into<String>) -> Option<&RuntimeValue> {
        let key = key.into();
        self.map.get(&key)
    }

    /// Insert or overwrite a value in the current environment only.
    pub fn set_value(&mut self, key: impl Into<String>, value: RuntimeValue) {
        let key = key.into();
        self.map.insert(key, value);
    }

    /// Set the parent runtime environment. The parent must be weak.
    pub fn set_parent(
        &mut self,
        runtime_environment: RuntimeEnvironment,
    ) -> Result<(), EnvironmentError> {
        if !runtime_environment.is_weak() {
            return Err(EnvironmentError::StrongParent);
        }
        self.parent = Some(runtime_environment);

        Ok(())
    }

    /// Set the child runtime environment using a strong reference.
    ///
    /// Returns an error if the provided environment has been dropped.
    pub fn set_child(
        &mut self,
        runtime_environment: RuntimeEnvironment,
    ) -> Result<(), EnvironmentError> {
        let Some(runtime_environment) = runtime_environment.upgrade() else {
            return Err(EnvironmentError::DroppedEnvironment);
        };
        self.child = Some(runtime_environment);

        Ok(())
    }
}

#[derive(Debug)]
/// Classification of environment handles for controlling ownership semantics.
pub enum RuntimeEnvironmentKind {
    /// A weak handle to an `Environment`, suitable for parent links.
    Weak(Weak<RefCell<Environment>>),
    /// A strong handle to an `Environment`, suitable for ownership.
    Strong(Rc<RefCell<Environment>>),
}

#[derive(Debug)]
/// Public handle for interacting with scoped runtime environments.
pub struct RuntimeEnvironment(RuntimeEnvironmentKind);

impl Default for RuntimeEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeEnvironment {
    /// Create a new, empty environment wrapped in a strong reference.
    pub fn new() -> Self {
        Self(RuntimeEnvironmentKind::Strong(Rc::new(RefCell::new(
            Environment::default(),
        ))))
    }

    /// Access the underlying `Rc<RefCell<Environment>>`, upgrading weak refs when needed.
    pub fn get(&self) -> Option<Rc<RefCell<Environment>>> {
        match &self.0 {
            RuntimeEnvironmentKind::Strong(rc) => Some(rc.clone()),
            RuntimeEnvironmentKind::Weak(weak) => weak.upgrade(),
        }
    }

    /// Retrieve a value by key, walking up through parent environments if necessary.
    pub fn get_env_value(&self, key: impl Into<String>) -> Option<RuntimeValue> {
        let key = key.into();

        let env_rc = self.get()?;
        let env = env_rc.borrow();

        if let Some(value) = env.get_value(&key) {
            Some(value.clone())
        } else if let Some(parent) = env.parent() {
            parent.get_env_value(&key)
        } else {
            None
        }
    }

    /// Insert a value into the current environment without traversing parents.
    ///
    /// Returns an error if the environment has been dropped.
    pub fn insert_env_value(
        &self,
        key: impl Into<String>,
        value: RuntimeValue,
    ) -> Result<(), EnvironmentError> {
        let key = key.into();

        let Some(env_rc) = self.get() else {
            return Err(EnvironmentError::DroppedEnvironment);
        };

        let mut env = env_rc.borrow_mut();
        env.set_value(key, value);

        Ok(())
    }

    /// Assign a value, replacing it in the nearest scope where it already exists.
    ///
    /// Returns an error if the key is not present in any reachable scope or
    /// if the environment has been dropped.
    pub fn assign_env_value(
        &self,
        key: impl Into<String>,
        value: RuntimeValue,
    ) -> Result<(), EnvironmentError> {
        let key = key.into();

        let Some(env_rc) = self.get() else {
            return Err(EnvironmentError::DroppedEnvironment);
        };

        let mut env = env_rc.borrow_mut();

        if env.get_value(&key).is_some() {
            env.set_value(key, value);
            Ok(())
        } else if let Some(parent) = env.parent() {
            parent.assign_env_value(key, value)
        } else {
            Err(EnvironmentError::KeyDoesNotExist(key))
        }
    }

    /// Check whether this runtime environment holds a weak reference.
    pub fn is_weak(&self) -> bool {
        matches!(self.0, RuntimeEnvironmentKind::Weak(_))
    }

    /// Convert to a strong environment handle if possible.
    pub fn upgrade(&self) -> Option<RuntimeEnvironment> {
        match &self.0 {
            RuntimeEnvironmentKind::Weak(weak) => weak
                .upgrade()
                .map(|rc| RuntimeEnvironment(RuntimeEnvironmentKind::Strong(rc))),
            RuntimeEnvironmentKind::Strong(rc) => Some(RuntimeEnvironment(
                RuntimeEnvironmentKind::Strong(rc.clone()),
            )),
        }
    }

    /// Convert to a weak environment handle.
    pub fn downgrade(&self) -> RuntimeEnvironment {
        match &self.0 {
            RuntimeEnvironmentKind::Strong(rc) => {
                RuntimeEnvironment(RuntimeEnvironmentKind::Weak(Rc::downgrade(rc)))
            }
            RuntimeEnvironmentKind::Weak(weak) => {
                RuntimeEnvironment(RuntimeEnvironmentKind::Weak(weak.clone()))
            }
        }
    }

    /// Get the child environment if present.
    pub fn get_child(&self) -> Option<RuntimeEnvironment> {
        let env_rc = self.get()?;
        let env = env_rc.borrow();
        env.child().cloned()
    }

    /// Get the parent environment if present.
    pub fn get_parent(&self) -> Option<RuntimeEnvironment> {
        let env_rc = self.get()?;
        let env = env_rc.borrow();
        env.parent().cloned()
    }

    /// Set the child environment using a strong handle.
    ///
    /// Returns an error if the provided environment has been dropped.
    pub fn set_child(
        &self,
        runtime_environment: RuntimeEnvironment,
    ) -> Result<(), EnvironmentError> {
        let Some(env_rc) = self.get() else {
            return Err(EnvironmentError::DroppedEnvironment);
        };

        let mut env = env_rc.borrow_mut();
        env.set_child(runtime_environment)?;

        Ok(())
    }

    /// Set the parent environment using a weak handle.
    ///
    /// Returns an error if the provided environment has been dropped.
    pub fn set_parent(
        &self,
        runtime_environment: RuntimeEnvironment,
    ) -> Result<(), EnvironmentError> {
        let Some(env_rc) = self.get() else {
            return Err(EnvironmentError::DroppedEnvironment);
        };

        let mut env = env_rc.borrow_mut();
        env.set_parent(runtime_environment)?;

        Ok(())
    }

    /// Create an empty child environment, link it to this environment, and return it.
    ///
    /// Returns an error if the environment has been dropped.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fabulist_lang::parser::error::SpanSlice;
    /// use fabulist_lang::interpreter::environment::RuntimeEnvironment;
    /// use fabulist_lang::interpreter::runtime_value::RuntimeValue;
    ///
    /// let root = RuntimeEnvironment::new();
    /// let child = root.add_empty_child().unwrap();
    ///
    /// child
    ///     .insert_env_value(
    ///         "x",
    ///         RuntimeValue::Number {
    ///             value: 1.0,
    ///             span_slice: SpanSlice::default(),
    ///         },
    ///     )
    ///     .unwrap();
    /// assert!(root.get_env_value("x").is_none());
    ///
    /// root
    ///     .insert_env_value(
    ///         "y",
    ///         RuntimeValue::Number {
    ///             value: 2.0,
    ///             span_slice: SpanSlice::default(),
    ///         },
    ///     )
    ///     .unwrap();
    /// let RuntimeValue::Number { value, .. } = child.get_env_value("y").unwrap() else {
    ///     panic!("expected number");
    /// };
    /// assert_eq!(value, 2.0);
    /// ```
    pub fn add_empty_child(&self) -> Result<RuntimeEnvironment, EnvironmentError> {
        let child_environment = RuntimeEnvironment::new();

        self.set_child(child_environment.clone())?;
        child_environment.set_parent(self.downgrade())?;

        Ok(child_environment)
    }
}

impl Clone for RuntimeEnvironment {
    fn clone(&self) -> Self {
        match &self.0 {
            RuntimeEnvironmentKind::Strong(rc) => {
                RuntimeEnvironment(RuntimeEnvironmentKind::Strong(rc.clone()))
            }
            RuntimeEnvironmentKind::Weak(weak) => {
                RuntimeEnvironment(RuntimeEnvironmentKind::Weak(weak.clone()))
            }
        }
    }
}
