use fabulist_derive::SyntaxTree;
use pest::{error::LineColLocation, iterators::Pair};

use crate::{
    ast::{
        dfn::{ObjectDfn, ParameterBodyDfn},
        expr::Expr,
        stmt::BlockStmt,
    },
    error::Error,
    parser::Rule,
};

#[derive(SyntaxTree, Debug, Clone)]
pub enum Primitive {
    #[production(object: ObjectDfn)]
    Object(ObjectPrimitive),

    #[production(expr: Expr)]
    Grouping(GroupingPrimitive),

    #[production(name: String)]
    Identifier(IdentifierPrimitive),

    #[production(parameters: ParameterBodyDfn, block_stmt: BlockStmt)]
    Lambda(LambdaPrimitive),

    #[production(identifiers: Vec<IdentifierPrimitive>)]
    Path(PathPrimitive),

    #[production]
    Context(ContextPrimitive),
}

impl TryFrom<Pair<'_, Rule>> for Primitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);

        match value.as_rule() {
            Rule::primitive_expr => match value.into_inner().next() {
                Some(inner) => Ok(Primitive::try_from(inner)?),
                None => Err(Error::map_span(value_span, "Invalid primitive expression")),
            },
            Rule::grouping => Ok(Primitive::Grouping(GroupingPrimitive::try_from(value)?)),
            Rule::identifier | Rule::strict_ident | Rule::raw_ident => {
                Ok(Primitive::Identifier(IdentifierPrimitive::try_from(value)?))
            }
            Rule::path => Ok(Primitive::Path(PathPrimitive::try_from(value)?)),
            Rule::object => Ok(Primitive::Object(ObjectPrimitive {
                lcol: value_lcol,
                object: ObjectDfn::try_from(value)?,
            })),
            Rule::lambda => Ok(Primitive::Lambda(LambdaPrimitive::try_from(value)?)),
            Rule::context => Ok(Primitive::Context(ContextPrimitive { lcol: value_lcol })),
            _ => Err(Error::map_span(value_span, "Invalid primitive expression")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for IdentifierPrimitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);

        match value.as_rule() {
            Rule::identifier => match value.into_inner().next() {
                Some(inner) => Ok(IdentifierPrimitive::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::strict_ident => match value.into_inner().next() {
                Some(inner) => Ok(IdentifierPrimitive::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::raw_ident => match value.into_inner().next() {
                Some(inner) => Ok(IdentifierPrimitive::try_from(inner)?),
                None => unreachable!(),
            },
            Rule::ident_interior => Ok(IdentifierPrimitive {
                lcol: value_lcol,
                name: value.as_str().to_string(),
            }),
            _ => Err(Error::map_span(value_span, "Invalid primitive")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for GroupingPrimitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);

        let expr = match value.into_inner().next() {
            Some(expr) => Ok(Expr::try_from(expr)?),
            None => Err(Error::map_span(value_span, "Expected expression")),
        }?;

        Ok(GroupingPrimitive {
            lcol: value_lcol,
            expr,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for PathPrimitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());
        let identifiers = value
            .into_inner()
            .map(IdentifierPrimitive::try_from)
            .collect::<Result<Vec<IdentifierPrimitive>, Error>>()?;

        Ok(PathPrimitive {
            lcol: value_lcol,
            identifiers,
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for LambdaPrimitive {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();
        let value_lcol = LineColLocation::from(value_span);
        let mut inner = value.into_inner();

        let parameters = match inner.find(|pair| pair.as_rule() == Rule::parameter_body) {
            Some(parameter_body_dfn) => ParameterBodyDfn::try_from(parameter_body_dfn),
            None => Err(Error::map_span(value_span, "Expected parameter body")),
        }?;

        let block_stmt = match inner.find(|pair| pair.as_rule() == Rule::block_stmt) {
            Some(block_stmt) => BlockStmt::try_from(block_stmt),
            None => Err(Error::map_span(value_span, "Expected a block statement")),
        }?;

        Ok(LambdaPrimitive {
            lcol: value_lcol,
            block_stmt,
            parameters,
        })
    }
}

#[cfg(test)]
mod primitive_expr_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_primitive_expr() {
        let test_helper = ParserTestHelper::<Primitive>::new(Rule::primitive_expr, "PrimitiveExpr");
        test_helper.assert_parse("ident");
        test_helper.assert_parse("r#module");
        test_helper.assert_parse("(ident)");
        test_helper.assert_parse("path::path2::path3");
        test_helper.assert_parse("{ \"key\": 5 }");
        test_helper.assert_parse("() => { goto module_1::part_1; }");
        test_helper.assert_parse("(param1, param2) => { let a = 5; }");
        test_helper.assert_parse("context");
    }
}
