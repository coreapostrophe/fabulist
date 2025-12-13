use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expr::models::{
        BooleanLiteral, CallExpr, ContextPrimitive, Expr, GroupingPrimitive, IdentifierPrimitive,
        LambdaPrimitive, Literal, LiteralPrimary, MemberExpr, NoneLiteral, NumberLiteral,
        ObjectPrimitive, PassUnary, PathPrimitive, Primary, PrimaryExpr, Primitive,
        PrimitivePrimary, StandardUnary, StringLiteral, Unary, UnaryOperator,
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
    type Output = Result<RuntimeValue, RuntimeError>;

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
        Ok(RuntimeValue::Identifier(self.name.clone()))
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
        todo!("Defer module implementations for last")
    }
}

impl Evaluable for ContextPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::Context)
    }
}

impl Evaluable for Primitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        match self {
            Primitive::Object(obj) => obj.evaluate(environment, context),
            Primitive::Grouping(group) => group.evaluate(environment, context),
            Primitive::Identifier(ident) => ident.evaluate(environment, context),
            Primitive::Lambda(lambda) => lambda.evaluate(environment, context),
            Primitive::Path(path) => path.evaluate(environment, context),
            Primitive::Context(ctx) => ctx.evaluate(environment, context),
        }
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

impl Evaluable for LiteralPrimary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        self.literal.evaluate(environment, context)
    }
}

impl Evaluable for PrimitivePrimary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        self.primitive.evaluate(environment, context)
    }
}

impl Evaluable for Primary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        match self {
            Primary::Literal(lit_primary) => lit_primary.evaluate(environment, context),
            Primary::Primitive(prim_primary) => prim_primary.evaluate(environment, context),
        }
    }
}

impl Evaluable for PrimaryExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        self.primary.evaluate(environment, context)
    }
}

impl Evaluable for StandardUnary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        match self.operator {
            UnaryOperator::Negation => {
                let RuntimeValue::Number(runtime_value) =
                    self.right.evaluate(environment, context)?
                else {
                    return Err(RuntimeError::UnaryNegationNonNumber(self.span.clone()));
                };
                Ok(RuntimeValue::Number(-runtime_value))
            }
            UnaryOperator::Not => {
                let RuntimeValue::Boolean(runtime_value) =
                    self.right.evaluate(environment, context)?
                else {
                    return Err(RuntimeError::UnaryNotNonBoolean(self.span.clone()));
                };
                Ok(RuntimeValue::Boolean(!runtime_value))
            }
        }
    }
}

impl Evaluable for PassUnary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        self.expr.evaluate(environment, context)
    }
}

impl Evaluable for Unary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        match self {
            Unary::Standard(standard_unary) => standard_unary.evaluate(environment, context),
            Unary::Pass(pass_unary) => pass_unary.evaluate(environment, context),
        }
    }
}

impl Evaluable for CallExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        let callee = self.callee.evaluate(environment, context)?;
        let args = self
            .argument_body
            .as_ref()
            .map(|arg| arg.evaluate(environment, context))
            .transpose()?
            .flatten()
            .unwrap_or_default();

        match callee {
            RuntimeValue::Lambda {
                parameters,
                body,
                closure,
            } => {
                let new_env = Environment::add_empty_child(&closure);

                if let Some(params) = parameters.parameters.as_ref() {
                    for (param, arg) in params.iter().zip(args.iter()) {
                        Environment::insert(&new_env, param.name.clone(), arg.clone());
                    }
                }

                let return_value = body.evaluate(&new_env, context)?;
                Ok(return_value)
            }
            RuntimeValue::Identifier(ident_name) => {
                match Environment::get_value(environment, &ident_name) {
                    Some(RuntimeValue::Lambda {
                        parameters,
                        body,
                        closure,
                    }) => {
                        let new_env = Environment::add_empty_child(&closure);

                        if let Some(params) = parameters.parameters.as_ref() {
                            for (param, arg) in params.iter().zip(args.iter()) {
                                Environment::insert(&new_env, param.name.clone(), arg.clone());
                            }
                        }

                        let return_value = body.evaluate(&new_env, context)?;
                        Ok(return_value)
                    }
                    Some(RuntimeValue::NativeFunction(func)) => Ok(func(args, self.span.clone())?),
                    _ => Err(RuntimeError::CallNonCallable(self.span.clone())),
                }
            }
            _ => Err(RuntimeError::CallNonCallable(self.span.clone())),
        }
    }
}

impl Evaluable for MemberExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        let left_value = self.left.evaluate(environment, context)?;

        self.members
            .iter()
            .try_fold(left_value, |current_value, member| {
                let member_env = Environment::add_empty_child(environment);

                if let RuntimeValue::Object(obj_map) = current_value {
                    for (key, value) in obj_map.iter() {
                        Environment::insert(&member_env, key.clone(), value.clone());
                    }
                }

                member.evaluate(&member_env, context)
            })
    }
}
