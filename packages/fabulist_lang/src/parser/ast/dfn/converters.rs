//! Converters from pest parse pairs into definition fragments.
use std::collections::HashMap;

use pest::iterators::Pair;

use crate::parser::{
    ast::expr::models::{Expr, IdentifierPrimitive},
    error::{ExtractSpanSlice, ParserError},
    Rule,
};

use super::models::{ArgumentBodyDfn, Dfn, ObjectDfn, ParameterBodyDfn};

impl TryFrom<Pair<'_, Rule>> for Dfn {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::object => Ok(Dfn::Object(ObjectDfn::try_from(value)?)),
            Rule::argument_body => Ok(Dfn::ArgumentBody(ArgumentBodyDfn::try_from(value)?)),
            Rule::parameter_body => Ok(Dfn::ParameterBody(ParameterBodyDfn::try_from(value)?)),
            _ => Err(ParserError::InvalidDefinition(value.extract_span_slice())),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ObjectDfn {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        let mut map = HashMap::<String, Expr>::new();

        if let Some(object_interior) = value.into_inner().next() {
            let obj_interior = object_interior.into_inner();
            let vec_pair = obj_interior.collect::<Vec<Pair<'_, Rule>>>();
            let chunked_pairs = vec_pair.chunks_exact(2);
            for key_value_pairs in chunked_pairs {
                let key = &key_value_pairs[0];
                let string_interior = match key.clone().into_inner().next() {
                    Some(interior) => interior,
                    None => unreachable!(),
                };
                let value = &key_value_pairs[1];
                map.insert(
                    string_interior.as_str().to_string(),
                    Expr::try_from(value.to_owned())?,
                );
            }
        }

        Ok(ObjectDfn {
            span_slice: value_span_slice,
            object: map,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for ArgumentBodyDfn {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        if let Some(arguments) = value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::arguments)
        {
            let arg_expr = arguments
                .into_inner()
                .map(Expr::try_from)
                .collect::<Result<Vec<Expr>, ParserError>>()?;
            Ok(ArgumentBodyDfn {
                span_slice: value_span_slice,
                arguments: Some(arg_expr),
            })
        } else {
            Ok(ArgumentBodyDfn {
                span_slice: value_span_slice,
                arguments: None,
            })
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ParameterBodyDfn {
    type Error = ParserError;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span_slice = value.extract_span_slice();

        if let Some(parameters) = value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::parameters)
        {
            let param_expr = parameters
                .into_inner()
                .map(IdentifierPrimitive::try_from)
                .collect::<Result<Vec<IdentifierPrimitive>, ParserError>>()?;
            Ok(ParameterBodyDfn {
                span_slice: value_span_slice,
                parameters: Some(param_expr),
            })
        } else {
            Ok(ParameterBodyDfn {
                span_slice: value_span_slice,
                parameters: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::ast::AstTestHelper, parser::Rule};

    use super::*;

    #[test]
    pub fn parses_parameter_body() {
        let test_helper =
            AstTestHelper::<ParameterBodyDfn>::new(Rule::parameter_body, "ParameterBodyDfn");
        test_helper.assert_parse(r#"(param1, param2, param3)"#);
    }

    #[test]
    pub fn parses_argument_body() {
        let test_helper =
            AstTestHelper::<ArgumentBodyDfn>::new(Rule::argument_body, "ArgumentBodyDfn");
        test_helper.assert_parse(r#"("string", 5, true)"#);
    }

    #[test]
    fn parses_object() {
        let test_helper = AstTestHelper::<ObjectDfn>::new(Rule::object, "ObjectDfn");
        test_helper.assert_parse(r#"{"boolean": false, "number": 10}"#);
    }
}
