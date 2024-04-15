use pest::iterators::Pair;

use crate::{ast::dfn::object::Object, parser::Rule};

use super::Error;

pub struct Meta(pub Object);

impl TryFrom<Pair<'_, Rule>> for Meta {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        if value.as_rule() == Rule::meta_statement {
            if let Some(object) = value
                .clone()
                .into_inner()
                .find(|pair| pair.as_rule() == Rule::object)
            {
                return Ok(Meta(Object::try_from(object)?));
            }
        }
        Err(Error::InvalidRule(value.as_rule()))
    }
}
