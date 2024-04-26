use std::collections::HashMap;

use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::expr::Expr, parser::Rule};

use super::Error;

#[derive(Debug, Clone)]
pub struct ObjectDfn {
    pub lcol: LineColLocation,
    pub map: HashMap<String, Expr>,
}

impl TryFrom<Pair<'_, Rule>> for ObjectDfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let object_dfn_lcol = LineColLocation::from(value.as_span());
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
            map,
            lcol: object_dfn_lcol,
        })
    }
}

#[cfg(test)]
mod object_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_object() {
        let test_helper = ParserTestHelper::<ObjectDfn>::new(Rule::object, "ObjectDfn");
        test_helper.assert_parse(r#"{"boolean": false, "number": 10}"#);
    }
}
