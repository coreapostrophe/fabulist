use pest::iterators::Pair;

use crate::{ast::dfn::object::Object, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct MetaStmt(pub Object);

impl TryFrom<Pair<'_, Rule>> for MetaStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        if value.as_rule() == Rule::meta_stmt {
            if let Some(object) = value
                .clone()
                .into_inner()
                .find(|pair| pair.as_rule() == Rule::object)
            {
                return Ok(MetaStmt(Object::try_from(object)?));
            }
        }
        Err(Error::InvalidRule(value.as_rule()))
    }
}

#[cfg(test)]
mod meta_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_meta_stmt() {
        let test_helper = ParserTestHelper::<MetaStmt>::new(Rule::meta_stmt, "MetaStmt");
        test_helper.assert_parse(r#"story { "start": "part-1" }"#);
    }
}
