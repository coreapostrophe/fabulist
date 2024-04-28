use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::expr::primitive::PrimitiveExpr, parser::Rule};

use super::Error;

#[derive(Debug, Clone)]
pub struct PathDfn {
    pub lcol: LineColLocation,
    pub identifiers: Vec<PrimitiveExpr>,
}

impl TryFrom<Pair<'_, Rule>> for PathDfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let pair_dfn_lcol = LineColLocation::from(value.as_span());
        let identifiers = value
            .into_inner()
            .map(|pair| {
                let primary = PrimitiveExpr::try_from(pair)?;
                match primary {
                    PrimitiveExpr::Identifier { .. } => Ok(primary),
                    _ => unreachable!(),
                }
            })
            .collect::<Result<Vec<PrimitiveExpr>, Error>>()?;

        Ok(PathDfn {
            identifiers,
            lcol: pair_dfn_lcol,
        })
    }
}
