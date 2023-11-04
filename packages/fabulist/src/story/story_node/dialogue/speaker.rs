pub struct Speaker {
    name: String,
}

impl Speaker {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod speaker_tests {
    use super::*;

    #[test]
    fn matches_use_spec() {
        let speaker = Speaker::new("mock_speaker");
        assert_eq!(speaker.name().as_str(), "mock_speaker");
    }
}