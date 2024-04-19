use pest::iterators::Pair;

use crate::{ast::expr::primary::PrimaryExpr, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct Path(pub Vec<PrimaryExpr>);

impl TryFrom<Pair<'_, Rule>> for Path {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let identifiers = value
            .into_inner()
            .map(|pair| {
                let pair_rule = pair.as_rule();
                let primary = PrimaryExpr::try_from(pair)?;
                match primary {
                    PrimaryExpr::Identifier(_) => Ok(primary),
                    _ => Err(Error::InvalidRule(pair_rule)),
                }
            })
            .collect::<Result<Vec<PrimaryExpr>, Error>>()?;

        Ok(Path(identifiers))
    }
}
