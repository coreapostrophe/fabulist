use self::quote::Quote;

pub mod quote;
pub mod speaker;

pub struct Dialogue {
    speaker: String,
    quotes: Vec<Quote>
}

impl Dialogue {
    pub fn speaker(&self) -> &String {
        &self.speaker
    }
    pub fn quotes(&self) -> &Vec<Quote> {
        &self.quotes
    }
    pub fn first_quote(&self) -> Option<&Quote> {
        self.quotes.first()
    }
}

pub struct DialogueBuilder {
    speaker: String,
    quotes: Vec<Quote>
}

impl DialogueBuilder {
    pub fn new(speaker: &str) -> Self {
        Self {
            speaker: speaker.to_string(),
            quotes: Vec::new()
        }
    }
    pub fn add_quote(mut self, quote: Quote) -> Self {
        self.quotes.push(quote);
        self
    }
    pub fn build(self) -> Dialogue {
        Dialogue {
            quotes: self.quotes,
            speaker: self.speaker
        }
    }
}

#[cfg(test)]
mod dialogue_tests {
    use crate::story::story_node::dialogue::quote::QuoteBuilder;

    use super::*;


    #[test]
    fn matches_use_spec() {
        let dialogue = DialogueBuilder::new("mock_speaker")
            .add_quote(QuoteBuilder::new("mock_quote_1").build())
            .add_quote(QuoteBuilder::new("mock_quote_2").build())
            .add_quote(QuoteBuilder::new("mock_quote_3").build())
            .build();
        assert_eq!(dialogue.speaker().as_str(), "mock_speaker");
        assert_eq!(dialogue.quotes().len(), 3);
    }
}