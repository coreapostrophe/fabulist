use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::expr::primary::PrimaryExpr, parser::Rule};

use super::Error;

#[derive(Debug)]
pub struct Path {
    pub lcol: LineColLocation,
    pub identifiers: Vec<PrimaryExpr>,
}

impl TryFrom<Pair<'_, Rule>> for Path {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let pair_lcol = LineColLocation::from(value.as_span());
        let identifiers = value
            .into_inner()
            .map(|pair| {
                let primary = PrimaryExpr::try_from(pair)?;
                match primary {
                    PrimaryExpr::Identifier(_) => Ok(primary),
                    _ => unreachable!(),
                }
            })
            .collect::<Result<Vec<PrimaryExpr>, Error>>()?;

        Ok(Path {
            identifiers,
            lcol: pair_lcol,
        })
    }
}
