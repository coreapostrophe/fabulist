use fabulist_derive::SyntaxTree;
use pest::{error::LineColLocation, iterators::Pair};

use crate::{error::Error, parser::Rule};

use self::{
    binary::BinaryExpr, call::CallExpr, member::MemberExpr, primary::Primary, unary::UnaryExpr,
};

pub mod binary;
pub mod call;
pub mod literal;
pub mod member;
pub mod primary;
pub mod primitive;
pub mod unary;

#[derive(SyntaxTree, Debug, Clone)]
pub enum Expr {
    #[production(primary: Primary)]
    Primary(Box<PrimaryExpr>),
    Unary(Box<UnaryExpr>),
    Call(Box<CallExpr>),
    Member(Box<MemberExpr>),
    Binary(Box<BinaryExpr>),
}

impl TryFrom<Pair<'_, Rule>> for Expr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_span = value.as_span();

        match value.as_rule() {
            Rule::expression => match value.into_inner().next() {
                Some(inner) => Ok(Expr::try_from(inner)?),
                None => Err(Error::map_span(
                    value_span,
                    "Unable to parse token tree interior",
                )),
            },

            Rule::unary_expr => Ok(UnaryExpr::try_from(value)?.into()),
            Rule::call_expr => Ok(CallExpr::try_from(value)?.into()),
            Rule::member_expr => Ok(MemberExpr::try_from(value)?.into()),

            Rule::logical_expr
            | Rule::equality_expr
            | Rule::comparison_expr
            | Rule::term_expr
            | Rule::factor_expr => Ok(BinaryExpr::try_from(value)?.into()),

            Rule::primary_expr
            | Rule::number
            | Rule::identifier
            | Rule::strict_ident
            | Rule::raw_ident
            | Rule::string
            | Rule::raw_string
            | Rule::path
            | Rule::object
            | Rule::lambda
            | Rule::grouping
            | Rule::boolean
            | Rule::none => Ok(PrimaryExpr::try_from(value)?.into()),
            _ => Err(Error::map_span(value_span, "Invalid expression")),
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for PrimaryExpr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_lcol = LineColLocation::from(value.as_span());
        Ok(PrimaryExpr {
            lcol: value_lcol,
            primary: Primary::try_from(value)?,
        })
    }
}

impl From<PrimaryExpr> for Expr {
    fn from(value: PrimaryExpr) -> Self {
        Expr::Primary(Box::new(value))
    }
}
