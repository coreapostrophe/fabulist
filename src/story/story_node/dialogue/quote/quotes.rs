use std::fmt::{Display, Formatter};

use crate::story::story_node::dialogue::quote::Quote;
use crate::util::vec_to_str;

#[derive(Default)]
pub struct Quotes(pub Vec<Quote>);

impl Quotes {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Display for Quotes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", vec_to_str(&self.0))
    }
}

#[cfg(test)]
mod quotes_test {
    use crate::story::story_node::dialogue::quote::quote_builder::QuoteBuilder;

    use super::*;

    #[test]
    fn it_displays() {
        let mut quotes = Quotes(Vec::new());
        quotes.0.push(QuoteBuilder::new(String::from("Quote 1")).build());
        quotes.0.push(QuoteBuilder::new(String::from("Quote 2")).build());
        quotes.0.push(QuoteBuilder::new(String::from("Quote 3")).build());

        assert_eq!(
            quotes.to_string(),
            "[Quote {text: \"Quote 1\"}, Quote {text: \"Quote 2\"}, Quote {text: \"Quote 3\"}]"
        );
    }
}
