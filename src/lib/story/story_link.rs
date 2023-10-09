use super::context::Context;

pub type NextClosure = fn(&Context) -> String;
pub type ChangeContextClosure = fn(&mut Context) -> ();

pub struct StoryLink {
    next: Option<NextClosure>,
    change_context: Option<ChangeContextClosure>,
}

impl StoryLink {
    pub fn new() -> Self {
        Self {
            next: None,
            change_context: None,
        }
    }
    pub fn next(&self) -> Option<NextClosure> {
        self.next
    }
    pub fn change_context(&self) -> Option<ChangeContextClosure> {
        self.change_context
    }
    pub fn set_next(&mut self, next: Option<NextClosure>) {
        self.next = next;
    }
    pub fn set_change_context(&mut self, change_context: Option<ChangeContextClosure>) {
        self.change_context = change_context;
    }
}
