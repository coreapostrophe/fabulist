use std::{fmt::Debug, marker::PhantomData};

use pest::{iterators::Pair, Parser};

use crate::parser::{GrammarParser, Rule};

pub mod decl;
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

pub struct ParserTestHelper<T> {
    rule_type: Rule,
    struct_name: String,
    phantom: PhantomData<T>,
}

impl<'a, T> ParserTestHelper<T>
where
    T: TryFrom<Pair<'a, Rule>> + Debug,
{
    pub fn new(rule_type: Rule, struct_name: impl Into<String>) -> Self {
        Self {
            rule_type,
            struct_name: struct_name.into(),
            phantom: PhantomData,
        }
    }
    pub fn assert_parse(&self, source: &'a str) -> T
    where
        T: TryFrom<Pair<'a, Rule>, Error = Error> + Debug,
    {
        let mut result =
            GrammarParser::parse(self.rule_type, source).expect("Failed to parse string.");
        let element = result.next().expect(&format!(
            "Failed to parse {} pair from string",
            self.struct_name
        ));
        let element_ast = T::try_from(element);
        assert!(element_ast.is_ok());
        element_ast.expect(&format!(
            "Failed to turn pair to `{}` struct",
            self.struct_name
        ))
    }
}
