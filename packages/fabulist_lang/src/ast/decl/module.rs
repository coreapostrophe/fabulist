use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::expr::primary::PrimaryExpr, parser::Rule};

use super::Error;

#[derive(Debug, Clone)]
pub struct ModDecl {
    pub lcol: LineColLocation,
    pub path: String,
    pub identifier: PrimaryExpr,
}

impl TryFrom<Pair<'_, Rule>> for ModDecl {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mod_decl_span = value.as_span();
        let mod_decl_lcol = LineColLocation::from(mod_decl_span);
        let mut inner = value.into_inner();

        let path = match inner
            .clone()
            .find(|pair| pair.as_node_tag() == Some("path"))
        {
            Some(path) => match PrimaryExpr::try_from(path)? {
                PrimaryExpr::String { value, .. } => Ok(value),
                _ => Err(Error::map_span(mod_decl_span, "Expected string")),
            },
            None => Err(Error::map_span(mod_decl_span, "Expected string file path")),
        }?;

        let identifier = match inner.find(|pair| pair.as_rule() == Rule::identifier) {
            Some(identifier) => PrimaryExpr::try_from(identifier),
            None => Err(Error::map_span(mod_decl_span, "Expected identifier")),
        }?;

        Ok(ModDecl {
            path,
            identifier,
            lcol: mod_decl_lcol,
        })
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
