use std::fmt::{Display, Formatter};

use crate::story::story_link::StoryLink;
use crate::story::story_node::dialogue::quote::quote_builder::QuoteBuilder;

pub mod quote_builder;
pub mod quotes;

pub struct Quote {
    text: String,
    response: Option<String>,
    story_link: StoryLink,
}

impl Quote {
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn response(&self) -> &Option<String> {
        &self.response
    }
    pub fn story_link(&self) -> &StoryLink {
        &self.story_link
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
    pub fn set_response(&mut self, response: Option<String>) {
        self.response = response;
    }
    pub fn set_story_link(&mut self, story_link: StoryLink) {
        self.story_link = story_link;
    }

    pub fn builder() -> QuoteBuilder {
        QuoteBuilder::default()
    }
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let response_str = match self.response() {
            Some(response) => format!(", response: \"{}\"", response.to_string()),
            _ => String::from("")
        };
        write!(
            f,
            "Quote {{text: \"{}\"{}}}",
            self.text,
            response_str
        )
    }
}

#[cfg(test)]
mod quote_tests {
    use super::*;

    #[test]
    fn it_displays() {
        let quote = QuoteBuilder::new(String::from("Hello"))
            .response(String::from("Hello, there!"))
            .build();
        assert_eq!(quote.to_string(), "Quote {text: \"Hello\", response: \"Hello, there!\"}")
    }
}
