use std::fmt::{Display, Formatter};

use crate::story::story_node::dialogue::dialogue_builder::DialogueBuilder;
use crate::story::story_node::dialogue::quote::Quote;
use crate::story::story_node::dialogue::quote::quotes::Quotes;

pub mod quote;
pub mod speaker;
pub mod dialogue_builder;

pub struct Dialogue {
    speaker: String,
    choice: Option<usize>,
    quotes: Quotes,
}

impl Dialogue {
    pub fn speaker(&self) -> &str {
        &self.speaker
    }
    pub fn choice(&self) -> Option<usize> {
        self.choice
    }
    pub fn quotes(&self) -> &Quotes {
        &self.quotes
    }
    pub fn mut_quotes(&mut self) -> &mut Quotes {
        &mut self.quotes
    }

    pub fn set_speaker(&mut self, speaker: String) {
        self.speaker = speaker;
    }
    pub fn set_choice(&mut self, choice: usize) {
        self.choice = Some(choice);
    }
    pub fn set_quotes(&mut self, quotes: Quotes) {
        self.quotes = quotes;
    }

    pub fn has_choices(&self) -> bool {
        self.quotes.0.len() > 1
    }
    pub fn first_quote(&self) -> Option<&Quote> {
        self.quotes.0.first()
    }

    pub fn builder() -> DialogueBuilder {
        DialogueBuilder::default()
    }
}

impl Display for Dialogue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let choice_str = match self.choice() {
            Some(choice) => format!(", choice: {}", choice.to_string()),
            _ => String::from("")
        };
        write!(
            f,
            "Dialogue {{speaker: \"{}\", quotes: {}{}}}",
            self.speaker,
            self.quotes,
            choice_str
        )
    }
}

#[cfg(test)]
mod dialogue_tests {
    use crate::story::story_node::dialogue::quote::quote_builder::QuoteBuilder;

    use super::*;

    #[test]
    fn it_displays() {
        let dialogue = DialogueBuilder::new(String::from("core"))
            .choice(1)
            .add_quote(QuoteBuilder::new(String::from("Hello!")).build())
            .add_quote(QuoteBuilder::new(String::from("Uhm, Hi.")).build())
            .add_quote(QuoteBuilder::new(String::from("Who are you?")).build())
            .build_classic();
        assert_eq!(
            dialogue.to_string(),
            "Dialogue {speaker: \"core\", \
            quotes: [Quote {text: \"Hello!\"}, \
            Quote {text: \"Uhm, Hi.\"}, \
            Quote {text: \"Who are you?\"}], \
            choice: 1}");
    }
}
