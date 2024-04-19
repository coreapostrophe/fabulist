use pest::iterators::Pair;

use crate::parser::Rule;

use super::{element::ElementStmt, Error};

#[derive(Debug)]
pub struct PartStmt {
    pub id: String,
    pub elements: Vec<ElementStmt>,
}

impl TryFrom<Pair<'_, Rule>> for PartStmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        let inner = value.into_inner();

        let id = match inner.find_first_tagged("id") {
            Some(id) => match id.into_inner().find_first_tagged("name") {
                Some(identifier) => Ok(identifier.as_str().to_string()),
                None => Err(Error::InvalidRule(value_rule)),
            },
            None => Err(Error::InvalidRule(value_rule)),
        }?;
        let elements = inner
            .filter(|pair| pair.as_rule() == Rule::element_decl)
            .map(|pair| ElementStmt::try_from(pair))
            .collect::<Result<Vec<ElementStmt>, Error>>()?;

        Ok(PartStmt { id, elements })
    }
}

#[cfg(test)]
mod part_stmt_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_part_stmt() {
        let test_helper = ParserTestHelper::<PartStmt>::new(Rule::part_stmt, "MetaStmt");
        test_helper.assert_parse(r#"#ident-1 [char]>"I'm a dialogue""#);
    }
}
