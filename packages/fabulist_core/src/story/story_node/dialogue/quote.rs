use crate::story::story_link::{ChangeContextClosure, NextClosure, StoryLink};

pub struct Quote {
    text: String,
    response: Option<String>,
    story_link: StoryLink,
}

impl Quote {
    pub fn text(&self) -> &String {
        &self.text
    }
    pub fn response(&self) -> Option<&String> {
        match self.response {
            Some(ref res) => Some(res),
            None => None,
        }
    }
    pub fn story_link(&self) -> &StoryLink {
        &self.story_link
    }
}

pub struct QuoteBuilder {
    text: String,
    response: Option<String>,
    story_link: StoryLink,
}

impl QuoteBuilder {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            response: None,
            story_link: StoryLink::new(),
        }
    }
    pub fn set_response(mut self, response: Option<&str>) -> Self {
        let unwrapped_response = match response {
            Some(response_string) => Some(response_string.to_string()),
            None => None,
        };
        self.response = unwrapped_response;
        self
    }
    pub fn set_next(mut self, next_closure: NextClosure) -> Self {
        self.story_link.set_next(next_closure);
        self
    }
    pub fn set_change_context(mut self, change_context_closure: ChangeContextClosure) -> Self {
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
mod quote_tests {
    use super::*;

    #[test]
    fn matches_use_spec() {
        let quote = QuoteBuilder::new("mock_text")
            .set_response(Some("mock_response"))
            .build();
        assert_eq!(quote.text().as_str(), "mock_text");
        assert_eq!(
            quote.response().expect("to have a response").as_str(),
            "mock_response"
        );
    }
}
