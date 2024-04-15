use pest::iterators::Pair;

use crate::parser::Rule;

use super::{element::ElementStmt, Error};

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
            .filter(|pair| pair.as_rule() == Rule::element)
            .map(|pair| ElementStmt::try_from(pair))
            .collect::<Result<Vec<ElementStmt>, Error>>()?;

        Ok(PartStmt { id, elements })
    }
}

#[cfg(test)]
mod part_stmt_tests {
    use pest::Parser;

    use crate::parser::GrammarParser;

    use super::*;

    fn parse_part_stmt(source: &str) -> PartStmt {
        let mut result =
            GrammarParser::parse(Rule::part_statement, source).expect("Failed to parse string.");
        let part = result.next().expect("Failed to parse part statement");
        let part_ast = PartStmt::try_from(part);
        assert!(part_ast.is_ok());
        part_ast.expect("Failed to turn pair to `PartStmt` struct")
    }

    #[test]
    fn parses_part_stmt() {
        parse_part_stmt(r#"#ident-1 [char]>"I'm a dialogue""#);
    }
}
