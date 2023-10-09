use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::story::story_node::dialogue::speaker::Speaker;
use crate::util::hashmap_to_str;

pub struct Speakers(pub HashMap<String, Speaker>);

impl Speakers {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn insert(&mut self, key: String, value: Speaker) {
        self.0.insert(key, value);
    }
    pub fn get(&self, key: &str) -> Option<&Speaker> {
        self.0.get(key)
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Speaker> {
        self.0.get_mut(key)
    }
}

impl Display for Speakers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StoryNodes {{{}}}", hashmap_to_str(&self.0))
    }
}
