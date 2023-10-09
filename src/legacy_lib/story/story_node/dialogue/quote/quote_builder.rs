use crate::story::story_link::{ChangeContextClosure, NextClosure, StoryLink};
use crate::story::story_node::dialogue::quote::Quote;

#[derive(Default)]
pub struct QuoteBuilder {
    text: String,
    response: Option<String>,
    story_link: StoryLink,
}

impl QuoteBuilder {
    pub fn new(text: String) -> Self {
        Self {
            text,
            response: None,
            story_link: StoryLink::new(),
        }
    }
    pub fn response(mut self, response: String) -> QuoteBuilder {
        self.response = Some(response);
        self
    }
    pub fn next(mut self, next_closure: NextClosure) -> QuoteBuilder {
        self.story_link.set_next(next_closure);
        self
    }
    pub fn change_context(mut self, change_context_closure: ChangeContextClosure) -> QuoteBuilder {
        self.story_link.set_change_context(change_context_closure);
        self
    }
    pub fn build(self) -> Quote {
        Quote {
            text: self.text,
            response: self.response,
            story_link: self.story_link,
        }
    }
}

#[cfg(test)]
mod quote_builder_tests {
    use super::*;

    #[test]
    fn it_builds() {
        let quote: Quote = QuoteBuilder::new(String::from("Hello"))
            .response(String::from("Hello, there!"))
            .build();
        assert_eq!(quote.text(), "Hello");
        assert_eq!(
            quote.response().as_ref().expect("Quote to have a response"),
            "Hello, there!"
        );
    }
}
