pub struct Speaker {
    name: String,
}

impl Speaker {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}
