use pest::iterators::Pair;

use crate::{ast::dfn::object::Object, parser::Rule};

use super::Error;

pub struct MetaStmt(pub Object);

impl TryFrom<Pair<'_, Rule>> for MetaStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        if value.as_rule() == Rule::meta_statement {
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
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_meta_stmt(source: &str) -> MetaStmt {
        let mut result =
            GrammarParser::parse(Rule::meta_statement, source).expect("Failed to parse string.");
        let meta = result.next().expect("Failed to parse call expression");
        let meta_ast = MetaStmt::try_from(meta);
        assert!(meta_ast.is_ok());
        meta_ast.expect("Failed to turn pair to `CallExpr` struct")
    }

    #[test]
    fn parses_meta_stmt() {
        parse_meta_stmt(r#"story { "start": "part-1" }"#);
    }
}
