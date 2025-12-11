use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{
    ast::expr::models::{Expr, IdentifierPrimitive},
    error::ParsingError,
    parser::Rule,
};

use super::models::{ArgumentBodyDfn, Dfn, ObjectDfn, ParameterBodyDfn};

impl TryFrom<Pair<'_, Rule>> for Dfn {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        match value.as_rule() {
            Rule::object => Ok(Dfn::Object(ObjectDfn::try_from(value)?)),
            Rule::argument_body => Ok(Dfn::ArgumentBody(ArgumentBodyDfn::try_from(value)?)),
            Rule::parameter_body => Ok(Dfn::ParameterBody(ParameterBodyDfn::try_from(value)?)),
            _ => Err(ParsingError::map_custom_error(
                value_span.into(),
                "Invalid definition",
            )),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ObjectDfn {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

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
            span: value_span.into(),
            object: map,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for ArgumentBodyDfn {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        if let Some(arguments) = value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::arguments)
        {
            let arg_expr = arguments
                .into_inner()
                .map(Expr::try_from)
                .collect::<Result<Vec<Expr>, pest::error::Error<Rule>>>()?;
            Ok(ArgumentBodyDfn {
                span: value_span.into(),
                arguments: Some(arg_expr),
            })
        } else {
            Ok(ArgumentBodyDfn {
                span: value_span.into(),
                arguments: None,
            })
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ParameterBodyDfn {
    type Error = pest::error::Error<Rule>;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        if let Some(parameters) = value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::parameters)
        {
            let param_expr = parameters
                .into_inner()
                .map(IdentifierPrimitive::try_from)
                .collect::<Result<Vec<IdentifierPrimitive>, pest::error::Error<Rule>>>()?;
            Ok(ParameterBodyDfn {
                span: value_span.into(),
                parameters: Some(param_expr),
            })
        } else {
            Ok(ParameterBodyDfn {
                span: value_span.into(),
                parameters: None,
            })
        }
    }
}
