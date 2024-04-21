use std::collections::HashMap;

use pest::iterators::Pair;

use crate::{ast::expr::Expr, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct Object(pub HashMap<String, Expr>);

impl TryFrom<Pair<'_, Rule>> for Object {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut object = HashMap::<String, Expr>::new();
        if let Some(object_interior) = value.into_inner().next() {
            let obj_interior = object_interior.into_inner();
            let vec_pair = obj_interior.collect::<Vec<Pair<'_, Rule>>>();
            let mut chunked_pairs = vec_pair.chunks_exact(2);
            while let Some(key_value_pairs) = chunked_pairs.next() {
                let key = &key_value_pairs[0];
                let string_interior = match key.clone().into_inner().next() {
                    Some(interior) => interior,
                    None => unreachable!(),
                };
                let value = &key_value_pairs[1];
                object.insert(
                    string_interior.as_str().to_string(),
                    Expr::try_from(value.to_owned())?,
                );
            }
        }
        Ok(Object(object))
    }
}

#[cfg(test)]
mod object_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_object() {
        let test_helper = ParserTestHelper::<Object>::new(Rule::object, "Object");
        test_helper.assert_parse(r#"{"boolean": false, "number": 10}"#);
    }
}
