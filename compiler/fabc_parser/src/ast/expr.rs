use fabc_lexer::tokens::Token;

use crate::{
    ast::{literal::Literal, primitive::Primitive},
    error::Error,
    Parsable, Parser,
};

pub mod binary;
pub mod grouping;
pub mod primary;
pub mod unary;

#[derive(Debug, PartialEq)]
pub enum Primary {
    Literal(Literal),
    Primitive(Primitive),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Primary(Primary),
    Grouping(Box<Expr>),
}

impl Parsable for Expr {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        parser.equality()
    }
}
