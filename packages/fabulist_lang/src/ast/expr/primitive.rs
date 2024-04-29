use pest::{error::LineColLocation, iterators::Pair};

use crate::{
    ast::dfn::{mutator::MutatorDfn, object::ObjectDfn, path::PathDfn},
    parser::Rule,
};

use super::{Error, Expr};

#[derive(Debug, Clone)]
pub enum PrimitiveExpr {
    Object {
        value: ObjectDfn,
        lcol: LineColLocation,
    },
    Grouping {
        value: Expr,
        lcol: LineColLocation,
    },
    Identifier {
        value: String,
        lcol: LineColLocation,
    },
    Mutator {
        value: MutatorDfn,
        lcol: LineColLocation,
    },
    Path {
        value: PathDfn,
        lcol: LineColLocation,
    },
    Context {
        lcol: LineColLocation,
    },
}

impl TryFrom<Pair<'_, Rule>> for PrimitiveExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let primitive_expr_span = value.as_span();
        let primitive_expr_lcol = LineColLocation::from(primitive_expr_span);

        match value.as_rule() {
            Rule::primitive_expr => match value.into_inner().next() {
                Some(inner) => Ok(PrimitiveExpr::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::identifier => match value.into_inner().next() {
                Some(inner) => Ok(PrimitiveExpr::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::grouping => match value.into_inner().next() {
                Some(expr) => Ok(PrimitiveExpr::Grouping {
                    value: Expr::try_from(expr)?,
                    lcol: primitive_expr_lcol,
                }),
                None => unreachable!(),
            },
            Rule::strict_ident => Ok(PrimitiveExpr::Identifier {
                value: value.as_str().to_string(),
                lcol: primitive_expr_lcol,
            }),
            Rule::raw_ident => match value.into_inner().next() {
                Some(interior) => Ok(PrimitiveExpr::Identifier {
                    value: interior.as_str().to_string(),
                    lcol: primitive_expr_lcol,
                }),
                None => unreachable!(),
            },
            Rule::path => Ok(PrimitiveExpr::Path {
                value: PathDfn::try_from(value)?,
                lcol: primitive_expr_lcol,
            }),
            Rule::object => Ok(PrimitiveExpr::Object {
                value: ObjectDfn::try_from(value)?,
                lcol: primitive_expr_lcol,
            }),
            Rule::mutator => Ok(PrimitiveExpr::Mutator {
                value: MutatorDfn::try_from(value)?,
                lcol: primitive_expr_lcol,
            }),
            Rule::context => Ok(PrimitiveExpr::Context {
                lcol: primitive_expr_lcol,
            }),
            _ => Err(Error::map_span(
                primitive_expr_span,
                "Invalid primitive expression",
            )),
        }
    }
}

#[cfg(test)]
mod primitive_expr_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_primitive_expr() {
        let test_helper =
            ParserTestHelper::<PrimitiveExpr>::new(Rule::primitive_expr, "PrimitiveExpr");
        test_helper.assert_parse("ident");
        test_helper.assert_parse("r#module");
        test_helper.assert_parse("(ident)");
        test_helper.assert_parse("path::path2::path3");
        test_helper.assert_parse("{ \"key\": 5 }");
        test_helper.assert_parse("|>{ goto module_1::part_1; }");
        test_helper.assert_parse("context");
    }
}
