use super::expr::{primitive::Primitive, Expr};
use crate::{error::Error, parser::Rule};
use fabulist_derive::SyntaxTree;
use pest::{error::LineColLocation, iterators::Pair};
use std::collections::HashMap;

pub mod lambda;
pub mod path;

#[derive(SyntaxTree, Debug, Clone)]
pub enum Dfn {
    #[production(arguments: Option<Vec<Expr>>)]
    ArgumentBody(ArgumentBodyDfn),
    #[production(parameters: Option<Vec<Primitive>>)]
    ParameterBody(ParameterBodyDfn),
    #[production(object: HashMap<String, Expr>)]
    Object(ObjectDfn),
}

impl TryFrom<Pair<'_, Rule>> for Dfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        match value.as_rule() {
            Rule::object => Ok(Dfn::Object(ObjectDfn::try_from(value)?)),
            Rule::argument_body => Ok(Dfn::ArgumentBody(ArgumentBodyDfn::try_from(value)?)),
            Rule::parameter_body => Ok(Dfn::ParameterBody(ParameterBodyDfn::try_from(value)?)),
            _ => Err(Error::map_span(value_span, "Invalid definition")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ObjectDfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());

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
            lcol: value_lcol,
            object: map,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for ArgumentBodyDfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());

        if let Some(arguments) = value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::arguments)
        {
            let arg_expr = arguments
                .into_inner()
                .map(Expr::try_from)
                .collect::<Result<Vec<Expr>, Error>>()?;
            Ok(ArgumentBodyDfn {
                lcol: value_lcol,
                arguments: Some(arg_expr),
            })
        } else {
            Ok(ArgumentBodyDfn {
                lcol: value_lcol,
                arguments: None,
            })
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for ParameterBodyDfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());

        if let Some(parameters) = value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::parameters)
        {
            let param_expr = parameters
                .into_inner()
                .map(|pair| {
                    let pair_span = pair.as_span();
                    let primitive_expr = Primitive::try_from(pair);
                    if let Ok(primitive_expr) = primitive_expr {
                        if let Primitive::Identifier { .. } = primitive_expr {
                            return Ok(primitive_expr);
                        }
                    }
                    Err(Error::map_span(pair_span, "Expected identifier"))
                })
                .collect::<Result<Vec<Primitive>, Error>>()?;
            Ok(ParameterBodyDfn {
                lcol: value_lcol,
                parameters: Some(param_expr),
            })
        } else {
            Ok(ParameterBodyDfn {
                lcol: value_lcol,
                parameters: None,
            })
        }
    }
}

#[cfg(test)]
mod dfn_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    pub fn parses_parameter_body() {
        let test_helper =
            ParserTestHelper::<ParameterBodyDfn>::new(Rule::parameter_body, "ParameterBodyDfn");
        test_helper.assert_parse(r#"(param1, param2, param3)"#);
    }

    #[test]
    pub fn parses_argument_body() {
        let test_helper =
            ParserTestHelper::<ArgumentBodyDfn>::new(Rule::argument_body, "ArgumentBodyDfn");
        test_helper.assert_parse(r#"("string", 5, true)"#);
    }

    #[test]
    fn parses_object() {
        let test_helper = ParserTestHelper::<ObjectDfn>::new(Rule::object, "ObjectDfn");
        test_helper.assert_parse(r#"{"boolean": false, "number": 10}"#);
    }
}
