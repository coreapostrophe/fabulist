use pest::iterators::Pair;

use crate::parser::Rule;

use self::{
    binary::BinaryExpr, call::CallExpr, member::MemberExpr, primary::PrimaryExpr, unary::UnaryExpr,
};

use super::Error;

pub mod binary;
pub mod call;
pub mod member;
pub mod primary;
pub mod unary;

#[derive(Debug)]
pub enum Expr {
    Primary(Box<PrimaryExpr>),
    Unary(Box<UnaryExpr>),
    Call(Box<CallExpr>),
    Member(Box<MemberExpr>),
    Binary(Box<BinaryExpr>),
}

impl From<PrimaryExpr> for Expr {
    fn from(value: PrimaryExpr) -> Self {
        Expr::Primary(Box::new(value))
    }
}

impl From<BinaryExpr> for Expr {
    fn from(value: BinaryExpr) -> Self {
        Expr::Binary(Box::new(value))
    }
}

impl From<UnaryExpr> for Expr {
    fn from(value: UnaryExpr) -> Self {
        Expr::Unary(Box::new(value))
    }
}

impl From<CallExpr> for Expr {
    fn from(value: CallExpr) -> Self {
        Expr::Call(Box::new(value))
    }
}

impl From<MemberExpr> for Expr {
    fn from(value: MemberExpr) -> Self {
        Expr::Member(Box::new(value))
    }
}

impl TryFrom<Pair<'_, Rule>> for Expr {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value_rule = value.as_rule();
        match value_rule {
            Rule::expression => match value.into_inner().next() {
                Some(inner) => Ok(Expr::try_from(inner)?),
                None => Err(Error::InvalidRule(value_rule)),
            },

            Rule::unary_expr => Ok(UnaryExpr::try_from(value)?.into()),
            Rule::call_expr => Ok(CallExpr::try_from(value)?.into()),
            Rule::member_expr => Ok(MemberExpr::try_from(value)?.into()),

            Rule::logical_expr => Ok(BinaryExpr::try_from(value)?.into()),
            Rule::equality_expr => Ok(BinaryExpr::try_from(value)?.into()),
            Rule::comparison_expr => Ok(BinaryExpr::try_from(value)?.into()),
            Rule::term_expr => Ok(BinaryExpr::try_from(value)?.into()),
            Rule::factor_expr => Ok(BinaryExpr::try_from(value)?.into()),

            Rule::primary_expr => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::number => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::identifier => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::strict_ident => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::raw_ident => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::string => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::raw_string => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::object => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::grouping => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::boolean => Ok(PrimaryExpr::try_from(value)?.into()),
            Rule::none => Ok(PrimaryExpr::try_from(value)?.into()),
            _ => Err(Error::InvalidRule(value_rule)),
        }
    }
}
