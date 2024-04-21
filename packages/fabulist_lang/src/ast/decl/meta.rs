use pest::iterators::Pair;

use crate::{ast::dfn::object::Object, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct MetaDecl(pub Object);

impl TryFrom<Pair<'_, Rule>> for MetaDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let meta_decl_span = value.as_span();
        match value
            .into_inner()
            .find(|pair| pair.as_rule() == Rule::object)
        {
            Some(object) => Ok(MetaDecl(Object::try_from(object)?)),
            None => Err(Error::map_span(
                meta_decl_span,
                "Expected object definition",
            )),
        }
    }
}

#[cfg(test)]
mod meta_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_meta_stmt() {
        let test_helper = ParserTestHelper::<MetaDecl>::new(Rule::meta_decl, "MetaDecl");
        test_helper.assert_parse(r#"story { "start": "part-1" }"#);
    }
}
