use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::expr::models::{
        AssignmentExpr, BinaryExpr, BinaryOperator, BooleanLiteral, CallExpr, ContextPrimitive,
        Expr, GroupingPrimitive, IdentifierPrimitive, LambdaPrimitive, Literal, LiteralPrimary,
        MemberExpr, NoneLiteral, NumberLiteral, ObjectPrimitive, PassUnary, PathPrimitive, Primary,
        PrimaryExpr, Primitive, PrimitivePrimary, StandardUnary, StringLiteral, Unary, UnaryExpr,
        UnaryOperator,
    },
    context::Context,
    environment::Environment,
    error::RuntimeError,
    interpreter::{runtime_value::RuntimeValue, Evaluable},
    intrinsics::{BooleanIntrinsics, NumberIntrinsics, ObjectIntrinsics, StringIntrinsics},
};

impl Evaluable for NumberLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::Number {
            value: self.value,
            span: self.span.clone(),
        })
    }
}

impl Evaluable for BooleanLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::Boolean {
            value: self.value,
            span: self.span.clone(),
        })
    }
}

impl Evaluable for StringLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::String {
            value: self.value.clone(),
            span: self.span.clone(),
        })
    }
}

impl Evaluable for NoneLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(RuntimeValue::None {
            span: self.span.clone(),
        })
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
        Ok(RuntimeValue::Identifier {
            name: self.name.clone(),
            span: self.span.clone(),
        })
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
            span: self.span.clone(),
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
        Ok(RuntimeValue::Context {
            span: self.span.clone(),
        })
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
                let RuntimeValue::Number {
                    value: runtime_value,
                    ..
                } = self.right.evaluate(environment, context)?
                else {
                    return Err(RuntimeError::UnaryNegationNonNumber(self.span.clone()));
                };
                Ok(RuntimeValue::Number {
                    value: -runtime_value,
                    span: self.span.clone(),
                })
            }
            UnaryOperator::Not => {
                let RuntimeValue::Boolean {
                    value: runtime_value,
                    ..
                } = self.right.evaluate(environment, context)?
                else {
                    return Err(RuntimeError::UnaryNotNonBoolean(self.span.clone()));
                };
                Ok(RuntimeValue::Boolean {
                    value: !runtime_value,
                    span: self.span.clone(),
                })
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

impl Evaluable for UnaryExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        self.unary.evaluate(environment, context)
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

        if self.argument_body.is_none() {
            return Ok(callee);
        }

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
                ..
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
            RuntimeValue::Identifier {
                name: ident_name, ..
            } => match Environment::get_value(environment, &ident_name) {
                Some(RuntimeValue::Lambda {
                    parameters,
                    body,
                    closure,
                    ..
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
            },
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

        if self.members.is_empty() {
            return Ok(left_value);
        }

        self.members
            .iter()
            .try_fold(left_value, |current_value, member| {
                let injected_env = match &current_value {
                    RuntimeValue::Number { .. } => {
                        Some(NumberIntrinsics::inject_intrinsics(environment))
                    }
                    RuntimeValue::Boolean { .. } => {
                        Some(BooleanIntrinsics::inject_intrinsics(environment))
                    }
                    RuntimeValue::String { .. } => {
                        Some(StringIntrinsics::inject_intrinsics(environment))
                    }
                    RuntimeValue::Object {
                        properties: obj_map,
                        ..
                    } => {
                        let injected_env = ObjectIntrinsics::inject_intrinsics(environment);

                        for (key, value) in obj_map.iter() {
                            Environment::insert(&injected_env, key.clone(), value.clone());
                        }

                        Some(injected_env)
                    }
                    _ => None,
                };

                match injected_env {
                    Some(injected_env) => member.evaluate(&injected_env, context),
                    None => member.evaluate(environment, context),
                }
            })
    }
}

impl Evaluable for BinaryExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        let left_value = self.left.evaluate(environment, context)?;

        let Some(operator) = &self.operator else {
            return Ok(left_value);
        };

        let Some(right_expr) = &self.right else {
            return Ok(left_value);
        };

        let right_value = right_expr.evaluate(environment, context)?;

        match operator {
            BinaryOperator::Addition => left_value + right_value,
            BinaryOperator::Subtraction => left_value - right_value,
            BinaryOperator::Multiply => left_value * right_value,
            BinaryOperator::Divide => left_value / right_value,
            BinaryOperator::EqualEqual => Ok(RuntimeValue::Boolean {
                value: left_value == right_value,
                span: self.span.clone(),
            }),
            BinaryOperator::NotEqual => Ok(RuntimeValue::Boolean {
                value: left_value != right_value,
                span: self.span.clone(),
            }),
            BinaryOperator::GreaterThan => Ok(RuntimeValue::Boolean {
                value: left_value > right_value,
                span: self.span.clone(),
            }),
            BinaryOperator::GreaterEqual => Ok(RuntimeValue::Boolean {
                value: left_value >= right_value,
                span: self.span.clone(),
            }),
            BinaryOperator::LessThan => Ok(RuntimeValue::Boolean {
                value: left_value < right_value,
                span: self.span.clone(),
            }),
            BinaryOperator::LessEqual => Ok(RuntimeValue::Boolean {
                value: left_value <= right_value,
                span: self.span.clone(),
            }),
            BinaryOperator::And => Ok(RuntimeValue::Boolean {
                value: left_value.to_bool()? && right_value.to_bool()?,
                span: self.span.clone(),
            }),
            BinaryOperator::Or => Ok(RuntimeValue::Boolean {
                value: left_value.to_bool()? || right_value.to_bool()?,
                span: self.span.clone(),
            }),
        }
    }
}

impl Evaluable for AssignmentExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        let left = self.left.evaluate(environment, context)?;

        let Some(right) = &self.right else {
            return Ok(left);
        };

        let RuntimeValue::Identifier { name, .. } = left else {
            return Err(RuntimeError::AssignmentToNonIdentifier(self.span.clone()));
        };

        Environment::insert(environment, name, right.evaluate(environment, context)?);

        Ok(RuntimeValue::None {
            span: self.span.clone(),
        })
    }
}

impl Evaluable for Expr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        match self {
            Expr::Primary(primary_expr) => primary_expr.evaluate(environment, context),
            Expr::Unary(unary_expr) => unary_expr.evaluate(environment, context),
            Expr::Call(call_expr) => call_expr.evaluate(environment, context),
            Expr::Member(member_expr) => member_expr.evaluate(environment, context),
            Expr::Binary(binary_expr) => binary_expr.evaluate(environment, context),
            Expr::Assignment(assignment_expr) => assignment_expr.evaluate(environment, context),
        }
    }
}

#[cfg(test)]
mod expr_evaluators_tests {

    use crate::{
        ast::{expr::models::PrimaryExpr, AssertEvaluateOptions, AstTestHelper},
        error::OwnedSpan,
        interpreter::runtime_value::RuntimeValue,
        parser::Rule,
    };

    #[test]
    fn evaluates_number_literal() {
        let test_helper = AstTestHelper::<PrimaryExpr>::new(Rule::primary_expr, "PrimaryExpr");

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "42",
                environment: None,
                context: None,
            })
            .expect("Failed to evaluate number literal");

        assert_eq!(
            result,
            RuntimeValue::Number {
                value: 42.0,
                span: result.span().clone(),
            }
        );
    }

    #[test]
    fn evaluates_boolean_literal() {
        let test_helper = AstTestHelper::<PrimaryExpr>::new(Rule::primary_expr, "PrimaryExpr");

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "true",
                environment: None,
                context: None,
            })
            .expect("Failed to evaluate boolean literal");

        assert_eq!(
            result,
            RuntimeValue::Boolean {
                value: true,
                span: result.span().clone(),
            }
        );
    }

    #[test]
    fn evaluates_string_literal() {
        let test_helper = AstTestHelper::<PrimaryExpr>::new(Rule::primary_expr, "PrimaryExpr");

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "\"hello world\"",
                environment: None,
                context: None,
            })
            .expect("Failed to evaluate string literal");

        assert_eq!(
            result,
            RuntimeValue::String {
                value: "hello world".to_string(),
                span: result.span().clone(),
            }
        );
    }

    #[test]
    fn evaluates_none_literal() {
        let test_helper = AstTestHelper::<PrimaryExpr>::new(Rule::primary_expr, "PrimaryExpr");

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "none",
                environment: None,
                context: None,
            })
            .expect("Failed to evaluate none literal");

        assert_eq!(
            result,
            RuntimeValue::None {
                span: result.span().clone(),
            }
        );
    }

    #[test]
    fn evaluates_object_primitive() {
        let test_helper = AstTestHelper::<PrimaryExpr>::new(Rule::primary_expr, "PrimaryExpr");
        let source = "{ \"a\": 1, \"b\": true, \"c\": \"test\" }".to_string();

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: &source,
                environment: None,
                context: None,
            })
            .expect("Failed to evaluate object primitive");

        let mut expected_properties = std::collections::HashMap::new();

        expected_properties.insert(
            "a".to_string(),
            RuntimeValue::Number {
                value: 1.0,
                span: OwnedSpan {
                    start: 7,
                    end: 8,
                    input: source.clone(),
                },
            },
        );
        expected_properties.insert(
            "b".to_string(),
            RuntimeValue::Boolean {
                value: true,
                span: OwnedSpan {
                    start: 15,
                    end: 19,
                    input: source.clone(),
                },
            },
        );
        expected_properties.insert(
            "c".to_string(),
            RuntimeValue::String {
                value: "test".to_string(),
                span: OwnedSpan {
                    start: 27,
                    end: 31,
                    input: source.clone(),
                },
            },
        );

        assert_eq!(
            result,
            RuntimeValue::Object {
                properties: expected_properties,
                span: OwnedSpan {
                    start: 0,
                    end: 34,
                    input: source.clone(),
                },
            }
        );
    }
}
