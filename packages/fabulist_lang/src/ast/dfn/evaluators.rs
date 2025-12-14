use std::collections::HashMap;

use crate::{
    ast::{
        dfn::models::{ArgumentBodyDfn, ObjectDfn, ParameterBodyDfn},
        expr::models::IdentifierPrimitive,
    },
    environment::RuntimeEnvironment,
    error::RuntimeError,
    interpreter::{runtime_value::RuntimeValue, Evaluable},
};

impl Evaluable for ObjectDfn {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
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
        _environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        Ok(self.parameters.clone())
    }
}

impl Evaluable for ArgumentBodyDfn {
    type Output = Result<Option<Vec<RuntimeValue>>, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
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

        let RuntimeValue::Number { value: arg_1, .. } = &result[0] else {
            panic!("Expected first argument to be a Number");
        };
        assert_eq!(*arg_1, 42.0);

        let RuntimeValue::String { value: arg_2, .. } = &result[1] else {
            panic!("Expected second argument to be a String");
        };
        assert_eq!(*arg_2, "hello".to_string());

        let RuntimeValue::Boolean { value: arg_3, .. } = &result[2] else {
            panic!("Expected third argument to be a Boolean");
        };
        assert!(*arg_3);
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

        assert_eq!(properties.len(), 3);

        let RuntimeValue::Number {
            value: key1_value, ..
        } = properties.get("key1").expect("Failed to get key1")
        else {
            panic!("Expected key1 to be a Number");
        };
        assert_eq!(*key1_value, 100.0);

        let RuntimeValue::String {
            value: key2_value, ..
        } = properties.get("key2").expect("Failed to get key2")
        else {
            panic!("Expected key2 to be a String");
        };
        assert_eq!(*key2_value, "value".to_string());

        let RuntimeValue::Boolean {
            value: key3_value, ..
        } = properties.get("key3").expect("Failed to get key3")
        else {
            panic!("Expected key3 to be a Boolean");
        };
        assert!(!*key3_value);
    }
}
