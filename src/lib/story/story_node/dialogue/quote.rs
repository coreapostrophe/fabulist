pub struct Quote {
    text: String,
    response: Option<String>,
}

impl Quote {
    pub fn new(text: String, response: Option<String>) -> Self {
        Self { text, response }
    }
    pub fn text(&self) -> &String {
        &self.text
    }
    pub fn response(&self) -> Option<&String> {
        match self.response {
            Some(ref res) => Some(res),
            None => None,
        }
    }
}
