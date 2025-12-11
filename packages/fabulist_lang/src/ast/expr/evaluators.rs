use std::{cell::RefCell, rc::Rc};

use fabulist_core::story::context::Context;

use crate::{
    ast::expr::models::{BooleanLiteral, Literal, NoneLiteral, NumberLiteral, StringLiteral},
    environment::Environment,
    error::RuntimeError,
    interpreter::Evaluable,
};

pub enum ExprValue {
    Number(f32),
    Boolean(bool),
    String(String),
    None,
}

impl Evaluable for NumberLiteral {
    type Output = Result<ExprValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(ExprValue::Number(self.value))
    }
}

impl Evaluable for BooleanLiteral {
    type Output = Result<ExprValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(ExprValue::Boolean(self.value))
    }
}

impl Evaluable for StringLiteral {
    type Output = Result<ExprValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(ExprValue::String(self.value.clone()))
    }
}

impl Evaluable for NoneLiteral {
    type Output = Result<ExprValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(ExprValue::None)
    }
}

impl Evaluable for Literal {
    type Output = Result<ExprValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        match self {
            Literal::Number(num_lit) => num_lit.evaluate(environment, context),
            Literal::Boolean(bool_lit) => bool_lit.evaluate(environment, context),
            Literal::String(str_lit) => str_lit.evaluate(environment, context),
            Literal::None(none_lit) => none_lit.evaluate(environment, context),
        }
    }
}
