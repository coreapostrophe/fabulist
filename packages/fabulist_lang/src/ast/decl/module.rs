use pest::iterators::Pair;

use crate::{ast::expr::primary::PrimaryExpr, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct ModDecl {
    pub path: String,
    pub identifier: PrimaryExpr,
}

impl TryFrom<Pair<'_, Rule>> for ModDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let mut inner = value.into_inner();

        let path = match inner
            .clone()
            .find(|pair| pair.as_node_tag() == Some("path"))
        {
            Some(path) => match PrimaryExpr::try_from(path)? {
                PrimaryExpr::String(string) => Ok(string),
                _ => Err(Error::InvalidRule(value_rule)),
            },
            None => Err(Error::InvalidRule(value_rule)),
        }?;

        let identifier = match inner.find(|pair| pair.as_rule() == Rule::identifier) {
            Some(identifier) => PrimaryExpr::try_from(identifier),
            None => Err(Error::InvalidRule(value_rule)),
        }?;

        Ok(ModDecl { path, identifier })
    }
}

#[cfg(test)]
mod module_decl_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_module_tests() {
        let test_helper = ParserTestHelper::<ModDecl>::new(Rule::mod_decl, "ModDecl");
        test_helper.assert_parse("module \"./module.fab\" as module_1;");
    }
}
