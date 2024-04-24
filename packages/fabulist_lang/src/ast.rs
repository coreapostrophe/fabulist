use std::{fmt::Debug, marker::PhantomData};

use pest::{iterators::Pair, Parser};

use crate::{
    error::Error,
    parser::{GrammarParser, Rule},
};

pub mod decl;
pub mod dfn;
pub mod expr;
pub mod stmt;
pub mod story;

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
        let pair = result.next().expect(&format!(
            "Failed to parse {} pair from string",
            self.struct_name
        ));
        let ast = T::try_from(pair);
        assert!(ast.is_ok());
        ast.expect(&format!(
            "Failed to turn pair to `{}` struct",
            self.struct_name
        ))
    }
}
