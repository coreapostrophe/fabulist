pub mod element;
pub mod meta;
pub mod part;

use pest::iterators::Pair;

use crate::parser::Rule;

use self::part::PartStmt;

use super::Error;

pub enum Stmt {
    Part(PartStmt),
}

impl From<PartStmt> for Stmt {
    fn from(value: PartStmt) -> Self {
        Self::Part(value)
    }
}

impl TryFrom<Pair<'_, Rule>> for Stmt {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        match value_rule {
            Rule::statement => match value.into_inner().next() {
                Some(inner) => Ok(Stmt::try_from(inner)?),
                None => Err(Error::InvalidRule(value_rule)),
            },
            Rule::part_statement => Ok(PartStmt::try_from(value)?.into()),
            _ => Err(Error::InvalidRule(value_rule)),
        }
    }
}
