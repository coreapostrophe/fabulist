use pest::{error::LineColLocation, iterators::Pair};

use crate::{ast::expr::primitive::Primitive, parser::Rule};

use super::Error;

#[derive(Debug, Clone)]
pub struct PathDfn {
    pub lcol: LineColLocation,
    pub identifiers: Vec<Primitive>,
}

impl TryFrom<Pair<'_, Rule>> for PathDfn {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let pair_dfn_lcol = LineColLocation::from(value.as_span());
        let identifiers = value
            .into_inner()
            .map(|pair| {
                let primary = Primitive::try_from(pair)?;
                match primary {
                    Primitive::Identifier { .. } => Ok(primary),
                    _ => unreachable!(),
                }
            })
            .collect::<Result<Vec<Primitive>, Error>>()?;

        Ok(PathDfn {
            identifiers,
            lcol: pair_dfn_lcol,
        })
    }
}
