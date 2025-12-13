use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::expr::models::{
        BooleanLiteral, Expr, GroupingPrimitive, IdentifierPrimitive, LambdaPrimitive, Literal,
        NoneLiteral, NumberLiteral, ObjectPrimitive, PathPrimitive, StringLiteral,
    },
    context::Context,
    environment::Environment,
    error::RuntimeError,
    interpreter::{Evaluable, RuntimeValue},
};

impl Evaluable for NumberLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::Number(self.value))
    }
}

impl Evaluable for BooleanLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::Boolean(self.value))
    }
}

impl Evaluable for StringLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::String(self.value.clone()))
    }
}

impl Evaluable for NoneLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::None)
    }
}

impl Evaluable for Literal {
    type Output = Result<RuntimeValue, RuntimeError>;

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

impl Evaluable for ObjectPrimitive {
    type Output = Result<HashMap<String, Expr>, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        self.object.evaluate(environment, context)
    }
}

impl Evaluable for GroupingPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        self.expr.evaluate(environment, context)
    }
}

impl Evaluable for IdentifierPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::String(self.name.clone()))
    }
}

impl Evaluable for LambdaPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::Lambda {
            parameters: self.parameters.clone(),
            body: self.block_stmt.clone(),
            closure: environment.clone(),
        })
    }
}

impl Evaluable for PathPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        todo!()
    }
}

impl Evaluable for Expr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        todo!()
    }
}
