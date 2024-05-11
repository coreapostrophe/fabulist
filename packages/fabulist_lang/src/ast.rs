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

pub struct AstTestHelper<T> {
    rule_type: Rule,
    struct_name: String,
    phantom: PhantomData<T>,
}

impl<'a, T> AstTestHelper<T>
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
        let pair = result
            .next()
            .unwrap_or_else(|| panic!("Failed to parse {} pair from string", self.struct_name));
        let ast = T::try_from(pair);
        assert!(ast.is_ok());
        ast.unwrap_or_else(|_| panic!("Failed to turn pair to `{}` struct", self.struct_name))
    }
}
