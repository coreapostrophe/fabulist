use fabc_lexer::tokens::Token;

use crate::ast::{literal::Literal, primitive::Primitive};

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
