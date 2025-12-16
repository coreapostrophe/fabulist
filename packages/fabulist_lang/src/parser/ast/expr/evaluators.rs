//! Expression evaluators that turn AST nodes into runtime values.
use crate::{
    interpreter::{
        environment::RuntimeEnvironment,
        error::RuntimeError,
        intrinsics::{BooleanIntrinsics, NumberIntrinsics, ObjectIntrinsics, StringIntrinsics},
        runtime_value::RuntimeValue,
        Evaluable,
    },
    parser::ast::expr::models::{
        AssignmentExpr, BinaryExpr, BinaryOperator, BooleanLiteral, CallExpr, ContextPrimitive,
        Expr, GroupingPrimitive, IdentifierPrimitive, LambdaPrimitive, Literal, LiteralPrimary,
        MemberExpr, NoneLiteral, NumberLiteral, ObjectPrimitive, PassUnary, PathPrimitive, Primary,
        PrimaryExpr, Primitive, PrimitivePrimary, StandardUnary, StringLiteral, Unary, UnaryExpr,
        UnaryOperator,
    },
};

impl Evaluable for NumberLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        Ok(RuntimeValue::Number {
            value: self.value,
            span_slice: self.span_slice.clone(),
        })
    }
}

impl Evaluable for BooleanLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        Ok(RuntimeValue::Boolean {
            value: self.value,
            span_slice: self.span_slice.clone(),
        })
    }
}

impl Evaluable for StringLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        Ok(RuntimeValue::String {
            value: self.value.clone(),
            span_slice: self.span_slice.clone(),
        })
    }
}

impl Evaluable for NoneLiteral {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        Ok(RuntimeValue::None {
            span_slice: self.span_slice.clone(),
        })
    }
}

impl Evaluable for Literal {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
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
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        self.object.evaluate(environment, context)
    }
}

impl Evaluable for GroupingPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        self.expr.evaluate(environment, context)
    }
}

impl Evaluable for IdentifierPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        Ok(RuntimeValue::Identifier {
            name: self.name.clone(),
            span_slice: self.span_slice.clone(),
        })
    }
}

impl Evaluable for LambdaPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        Ok(RuntimeValue::Lambda {
            parameters: self.parameters.clone(),
            body: self.block_stmt.clone(),
            closure: environment.clone(),
            span_slice: self.span_slice.clone(),
        })
    }
}

impl Evaluable for PathPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        todo!("Defer module implementations for last")
    }
}

impl Evaluable for ContextPrimitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        Ok(RuntimeValue::Context {
            span_slice: self.span_slice.clone(),
        })
    }
}

impl Evaluable for Primitive {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
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
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        self.literal.evaluate(environment, context)
    }
}

impl Evaluable for PrimitivePrimary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        self.primitive.evaluate(environment, context)
    }
}

impl Evaluable for Primary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
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
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        self.primary.evaluate(environment, context)
    }
}

impl Evaluable for StandardUnary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        match self.operator {
            UnaryOperator::Negation => match self.right.evaluate(environment, context)? {
                RuntimeValue::Number {
                    value: runtime_value,
                    ..
                } => Ok(RuntimeValue::Number {
                    value: -runtime_value,
                    span_slice: self.span_slice.clone(),
                }),
                RuntimeValue::Identifier {
                    name,
                    span_slice: span,
                } => {
                    let value = environment
                        .get_env_value(&name)
                        .ok_or(RuntimeError::InvalidIdentifier(span.clone()))?;

                    match value {
                        RuntimeValue::Number {
                            value: runtime_value,
                            ..
                        } => Ok(RuntimeValue::Number {
                            value: -runtime_value,
                            span_slice: self.span_slice.clone(),
                        }),
                        other => Err(RuntimeError::UnaryNegationNonNumber(other.span().clone())),
                    }
                }
                other => Err(RuntimeError::UnaryNegationNonNumber(other.span().clone())),
            },
            UnaryOperator::Not => match self.right.evaluate(environment, context)? {
                RuntimeValue::Boolean {
                    value: runtime_value,
                    ..
                } => Ok(RuntimeValue::Boolean {
                    value: !runtime_value,
                    span_slice: self.span_slice.clone(),
                }),
                RuntimeValue::Identifier {
                    name,
                    span_slice: span,
                } => {
                    let value = environment
                        .get_env_value(&name)
                        .ok_or(RuntimeError::InvalidIdentifier(span.clone()))?;

                    match value {
                        RuntimeValue::Boolean {
                            value: runtime_value,
                            ..
                        } => Ok(RuntimeValue::Boolean {
                            value: !runtime_value,
                            span_slice: self.span_slice.clone(),
                        }),
                        other => Err(RuntimeError::UnaryNotNonBoolean(other.span().clone())),
                    }
                }
                other => Err(RuntimeError::UnaryNotNonBoolean(other.span().clone())),
            },
        }
    }
}

impl Evaluable for PassUnary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        self.expr.evaluate(environment, context)
    }
}

impl Evaluable for Unary {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
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
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        self.unary.evaluate(environment, context)
    }
}

impl Evaluable for CallExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
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
                let new_env = closure.add_empty_child()?;

                if let Some(params) = parameters.parameters.as_ref() {
                    for (param, arg) in params.iter().zip(args.iter()) {
                        new_env.insert_env_value(param.name.clone(), arg.clone())?;
                    }
                }

                let return_value = body.evaluate(&new_env, context)?;
                Ok(return_value)
            }
            RuntimeValue::Identifier {
                name: ident_name, ..
            } => match environment.get_env_value(&ident_name) {
                Some(RuntimeValue::Lambda {
                    parameters,
                    body,
                    closure,
                    ..
                }) => {
                    let new_env = closure.add_empty_child()?;

                    if let Some(params) = parameters.parameters.as_ref() {
                        for (param, arg) in params.iter().zip(args.iter()) {
                            new_env.insert_env_value(param.name.clone(), arg.clone())?;
                        }
                    }

                    let return_value = body.evaluate(&new_env, context)?;
                    Ok(return_value)
                }
                Some(RuntimeValue::NativeFunction(func)) => {
                    Ok(func(args, self.span_slice.clone())?)
                }
                _ => Err(RuntimeError::CallNonCallable(self.span_slice.clone())),
            },
            _ => Err(RuntimeError::CallNonCallable(self.span_slice.clone())),
        }
    }
}

impl Evaluable for MemberExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        let left_value = self.left.evaluate(environment, context)?;

        if self.members.is_empty() {
            return Ok(left_value);
        }

        self.members
            .iter()
            .try_fold(left_value, |current_value, member| {
                let injected_env = match &current_value {
                    RuntimeValue::Number { .. } => NumberIntrinsics::inject_intrinsics(environment),
                    RuntimeValue::Boolean { .. } => {
                        BooleanIntrinsics::inject_intrinsics(environment)
                    }
                    RuntimeValue::String { .. } => StringIntrinsics::inject_intrinsics(environment),
                    RuntimeValue::Object {
                        properties: obj_map,
                        ..
                    } => {
                        let injected_env = ObjectIntrinsics::inject_intrinsics(environment)?;

                        for (key, value) in obj_map.iter() {
                            injected_env.insert_env_value(key, value.clone())?;
                        }

                        Ok(injected_env)
                    }
                    RuntimeValue::Context { .. } => Ok(context.clone()),
                    RuntimeValue::Module { environment, .. } => Ok(environment.clone()),
                    other => return Err(RuntimeError::InvalidMemoryAccess(other.span().clone())),
                }?;

                member.evaluate(&injected_env, context)
            })
    }
}

impl Evaluable for BinaryExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        let mut left_value = self.left.evaluate(environment, context)?;

        if let RuntimeValue::Identifier {
            name,
            span_slice: span,
        } = &left_value
        {
            if let Some(value) = environment.get_env_value(name) {
                left_value = value;
            } else {
                return Err(RuntimeError::InvalidIdentifier(span.clone()));
            }
        }

        let Some(operator) = &self.operator else {
            return Ok(left_value);
        };

        let Some(right_expr) = &self.right else {
            return Ok(left_value);
        };

        let mut right_value = right_expr.evaluate(environment, context)?;

        if let RuntimeValue::Identifier {
            name,
            span_slice: span,
        } = &right_value
        {
            if let Some(value) = environment.get_env_value(name) {
                right_value = value;
            } else {
                return Err(RuntimeError::InvalidIdentifier(span.clone()));
            }
        }

        match operator {
            BinaryOperator::Addition => left_value + right_value,
            BinaryOperator::Subtraction => left_value - right_value,
            BinaryOperator::Multiply => left_value * right_value,
            BinaryOperator::Divide => left_value / right_value,
            BinaryOperator::EqualEqual => Ok(RuntimeValue::Boolean {
                value: left_value == right_value,
                span_slice: self.span_slice.clone(),
            }),
            BinaryOperator::NotEqual => Ok(RuntimeValue::Boolean {
                value: left_value != right_value,
                span_slice: self.span_slice.clone(),
            }),
            BinaryOperator::GreaterThan => Ok(RuntimeValue::Boolean {
                value: left_value > right_value,
                span_slice: self.span_slice.clone(),
            }),
            BinaryOperator::GreaterEqual => Ok(RuntimeValue::Boolean {
                value: left_value >= right_value,
                span_slice: self.span_slice.clone(),
            }),
            BinaryOperator::LessThan => Ok(RuntimeValue::Boolean {
                value: left_value < right_value,
                span_slice: self.span_slice.clone(),
            }),
            BinaryOperator::LessEqual => Ok(RuntimeValue::Boolean {
                value: left_value <= right_value,
                span_slice: self.span_slice.clone(),
            }),
            BinaryOperator::And => Ok(RuntimeValue::Boolean {
                value: left_value.to_bool()? && right_value.to_bool()?,
                span_slice: self.span_slice.clone(),
            }),
            BinaryOperator::Or => Ok(RuntimeValue::Boolean {
                value: left_value.to_bool()? || right_value.to_bool()?,
                span_slice: self.span_slice.clone(),
            }),
        }
    }
}

impl Evaluable for AssignmentExpr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        let left = self.left.evaluate(environment, context)?;

        let Some(right) = &self.right else {
            return Ok(left);
        };

        let RuntimeValue::Identifier { name, .. } = left else {
            return Err(RuntimeError::AssignmentToNonIdentifier(
                self.span_slice.clone(),
            ));
        };

        let right_value = right.evaluate(environment, context)?;
        environment.assign_env_value(&name, right_value)?;

        Ok(RuntimeValue::None {
            span_slice: self.span_slice.clone(),
        })
    }
}

impl Evaluable for Expr {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
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
        interpreter::runtime_value::RuntimeValue,
        parser::{
            ast::{expr::models::PrimaryExpr, AssertEvaluateOptions, AstTestHelper},
            error::SpanSlice,
            Rule,
        },
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

        let RuntimeValue::Number {
            value: num_value, ..
        } = &result
        else {
            panic!("Expected a Number runtime value");
        };
        assert_eq!(*num_value, 42.0);
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

        let RuntimeValue::Boolean {
            value: bool_value, ..
        } = &result
        else {
            panic!("Expected a Boolean runtime value");
        };
        assert!(*bool_value);
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

        let RuntimeValue::String {
            value: string_value,
            ..
        } = &result
        else {
            panic!("Expected a String runtime value");
        };
        assert_eq!(*string_value, "hello world".to_string());
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
                span_slice: result.span().clone(),
            }
        );
    }

    #[test]
    fn evaluates_object_primitive() {
        let test_helper = AstTestHelper::<PrimaryExpr>::new(Rule::primary_expr, "PrimaryExpr");

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "{ \"a\": 1, \"b\": true, \"c\": \"test\" }",
                environment: None,
                context: None,
            })
            .expect("Failed to evaluate object primitive");

        let mut expected_properties = std::collections::HashMap::new();

        expected_properties.insert(
            "a".to_string(),
            RuntimeValue::Number {
                value: 1.0,
                span_slice: SpanSlice::default(),
            },
        );
        expected_properties.insert(
            "b".to_string(),
            RuntimeValue::Boolean {
                value: true,
                span_slice: SpanSlice::default(),
            },
        );
        expected_properties.insert(
            "c".to_string(),
            RuntimeValue::String {
                value: "test".to_string(),
                span_slice: SpanSlice::default(),
            },
        );

        let RuntimeValue::Object { properties, .. } = &result else {
            panic!("Expected a Object runtime value");
        };
        assert_eq!(*properties, expected_properties);
    }
}
