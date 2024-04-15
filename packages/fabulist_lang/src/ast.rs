use crate::parser::Rule;

pub mod ctrl_stmt;
pub mod dfn;
pub mod expr;
pub mod stmt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(
        "Binary expressions can only have the following operators: 
        [ /,*,+,-,>,>=,<,<=,==,!=,&&,|| ]"
    )]
    InvalidBinaryOperator,
    #[error(
        "Unary expressions can only have the following operators: 
        [ +,- ]"
    )]
    InvalidUnaryOperator,
    #[error("`start` can only be of type `String`")]
    InvalidStart,
    #[error("Unable to parse `{0}` to number.")]
    InvalidNumber(String),
    #[error("Token pair does not match rule `{0:?}`")]
    InvalidRule(Rule),
}
