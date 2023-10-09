use std::fmt::{Display, Formatter};

pub mod speakers;

#[derive(Debug, PartialEq)]
pub struct Speaker {
    name: String,
}

impl Speaker {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Display for Speaker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Conversant {{ name: {} }}",
            self.name
        )
    }
}

#[cfg(test)]
mod conversant_tests {
    use super::*;

    #[test]
    fn it_displays() {
        let conversant = Speaker::new(String::from("Test Name"));
        assert_eq!(conversant.to_string(), String::from("Conversant { name: Test Name }"))
    }
}
