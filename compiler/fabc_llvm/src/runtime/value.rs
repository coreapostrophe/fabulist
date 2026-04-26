use std::{cell::RefCell, collections::BTreeMap, fmt, rc::Rc};

use crate::ir::FunctionId;

use super::{error::RuntimeError, scope::Scope};

pub type ObjectRef = Rc<RefCell<BTreeMap<String, Value>>>;

#[derive(Clone, Debug)]
pub struct ClosureValue {
    pub function_id: FunctionId,
    pub captured: Scope,
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    None,
    Object(ObjectRef),
    Closure(ClosureValue),
    StoryRef(String),
}

impl Value {
    pub fn object(properties: BTreeMap<String, Value>) -> Self {
        Self::Object(Rc::new(RefCell::new(properties)))
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "Number",
            Value::Boolean(_) => "Boolean",
            Value::String(_) => "String",
            Value::None => "None",
            Value::Object(_) => "Object",
            Value::Closure(_) => "Closure",
            Value::StoryRef(_) => "StoryRef",
        }
    }

    pub fn to_bool(&self) -> Result<bool, RuntimeError> {
        match self {
            Value::Boolean(value) => Ok(*value),
            Value::Number(value) => Ok(*value != 0.0),
            Value::String(value) => Ok(!value.is_empty()),
            Value::None => Ok(false),
            other => Err(RuntimeError::InvalidBooleanCast(
                other.kind_name().to_string(),
            )),
        }
    }

    pub fn to_number(&self) -> Result<f64, RuntimeError> {
        match self {
            Value::Number(value) => Ok(*value),
            Value::Boolean(value) => Ok(if *value { 1.0 } else { 0.0 }),
            Value::None => Ok(0.0),
            Value::String(value) => value
                .parse::<f64>()
                .map_err(|_| RuntimeError::InvalidNumberCast(self.kind_name().to_string())),
            other => Err(RuntimeError::InvalidNumberCast(
                other.kind_name().to_string(),
            )),
        }
    }

    pub fn to_member_key(&self) -> Result<String, RuntimeError> {
        match self {
            Value::String(value) => Ok(value.clone()),
            Value::StoryRef(value) => Ok(value.clone()),
            other => Err(RuntimeError::InvalidMemberKey(
                other.kind_name().to_string(),
            )),
        }
    }

    pub fn to_story_target(&self) -> Result<String, RuntimeError> {
        match self {
            Value::String(value) => Ok(value.clone()),
            Value::StoryRef(value) => Ok(value.clone()),
            other => Err(RuntimeError::InvalidStoryTarget(
                other.kind_name().to_string(),
            )),
        }
    }

    pub fn add(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        match self {
            Value::String(left) => Ok(Value::String(format!("{left}{}", rhs.display_value()))),
            _ => Ok(Value::Number(self.to_number()? + rhs.to_number()?)),
        }
    }

    pub fn subtract(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.to_number()? - rhs.to_number()?))
    }

    pub fn multiply(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.to_number()? * rhs.to_number()?))
    }

    pub fn divide(&self, rhs: &Value) -> Result<Value, RuntimeError> {
        Ok(Value::Number(self.to_number()? / rhs.to_number()?))
    }

    fn display_value(&self) -> String {
        match self {
            Value::Number(value) => value.to_string(),
            Value::Boolean(value) => value.to_string(),
            Value::String(value) => value.clone(),
            Value::None => "none".to_string(),
            Value::Object(_) => "[object]".to_string(),
            Value::Closure(_) => "[closure]".to_string(),
            Value::StoryRef(value) => value.clone(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => left == right,
            (Value::Boolean(left), Value::Boolean(right)) => left == right,
            (Value::String(left), Value::String(right)) => left == right,
            (Value::None, Value::None) => true,
            (Value::StoryRef(left), Value::StoryRef(right)) => left == right,
            (Value::Object(left), Value::Object(right)) => *left.borrow() == *right.borrow(),
            (Value::Closure(left), Value::Closure(right)) => left.function_id == right.function_id,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.display_value())
    }
}
