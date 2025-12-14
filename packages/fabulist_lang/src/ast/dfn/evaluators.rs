use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::{
        dfn::models::{ArgumentBodyDfn, ObjectDfn, ParameterBodyDfn},
        expr::models::IdentifierPrimitive,
    },
    context::Context,
    environment::Environment,
    error::RuntimeError,
    interpreter::{runtime_value::RuntimeValue, Evaluable},
};

impl Evaluable for ObjectDfn {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        self.object
            .clone()
            .into_iter()
            .map(|(key, expr)| {
                let value = expr.evaluate(environment, context)?;
                Ok((key.clone(), value))
            })
            .collect::<Result<HashMap<String, RuntimeValue>, RuntimeError>>()
            .map(|properties| RuntimeValue::Object {
                properties,
                span: self.span.clone(),
            })
    }
}

impl Evaluable for ParameterBodyDfn {
    type Output = Result<Option<Vec<IdentifierPrimitive>>, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &Rc<RefCell<Environment>>,
        _context: &mut Context,
    ) -> Self::Output {
        Ok(self.parameters.clone())
    }
}

impl Evaluable for ArgumentBodyDfn {
    type Output = Result<Option<Vec<RuntimeValue>>, RuntimeError>;

    fn evaluate(
        &self,
        environment: &Rc<RefCell<Environment>>,
        context: &mut Context,
    ) -> Self::Output {
        match &self.arguments {
            Some(args) => args
                .iter()
                .map(|arg| arg.evaluate(environment, context))
                .collect::<Result<Vec<RuntimeValue>, RuntimeError>>()
                .map(Some),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod dfn_evaluators_tests {
    use crate::{
        ast::{AssertEvaluateOptions, AstTestHelper},
        parser::Rule,
    };

    use super::*;

    #[test]
    fn evaluates_parameter_body_dfn() {
        let test_helper =
            AstTestHelper::<ParameterBodyDfn>::new(Rule::parameter_body, "ParameterBodyDfn");

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "(param1, param2, param3)",
                environment: None,
                context: None,
            })
            .expect("Failed to evaluate ParameterBodyDfn")
            .expect("Expected Some parameters");

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name, "param1");
        assert_eq!(result[1].name, "param2");
        assert_eq!(result[2].name, "param3");
    }

    #[test]
    fn evaluates_argument_body_dfn() {
        let test_helper =
            AstTestHelper::<ArgumentBodyDfn>::new(Rule::argument_body, "ArgumentBodyDfn");

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "(42, \"hello\", true)",
                environment: None,
                context: None,
            })
            .expect("Failed to evaluate ArgumentBodyDfn")
            .expect("Expected Some arguments");

        assert_eq!(result.len(), 3);
        assert_eq!(
            result[0],
            RuntimeValue::Number {
                value: 42.0,
                span: result[0].span().clone()
            }
        );
        assert_eq!(
            result[1],
            RuntimeValue::String {
                value: "hello".to_string(),
                span: result[1].span().clone()
            }
        );
        assert_eq!(
            result[2],
            RuntimeValue::Boolean {
                value: true,
                span: result[2].span().clone()
            }
        );
    }

    #[test]
    fn evaluates_object_dfn() {
        let test_helper = AstTestHelper::<ObjectDfn>::new(Rule::object, "ObjectDfn");

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "{ \"key1\": 100, \"key2\": \"value\", \"key3\": false }",
                environment: None,
                context: None,
            })
            .expect("Failed to evaluate ObjectDfn");

        let RuntimeValue::Object { properties, .. } = &result else {
            panic!("Expected RuntimeValue::Object");
        };

        let key1_value = properties.get("key1").expect("Failed to get key1");
        assert_eq!(properties.len(), 3);
        assert_eq!(
            key1_value,
            &RuntimeValue::Number {
                value: 100.0,
                span: key1_value.span().clone(),
            }
        );

        let key2_value = properties.get("key2").expect("Failed to get key2");
        assert_eq!(
            key2_value,
            &RuntimeValue::String {
                value: "value".to_string(),
                span: key2_value.span().clone(),
            }
        );

        let key3_value = properties.get("key3").expect("Failed to get key3");
        assert_eq!(
            key3_value,
            &RuntimeValue::Boolean {
                value: false,
                span: key3_value.span().clone()
            }
        );
    }
}
