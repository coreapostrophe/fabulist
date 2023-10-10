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
    pub fn next(&self) -> Option<&NextClosure> {
        match self.next {
            Some(ref next_closure) => Some(next_closure),
            None => None
        }
    }
    pub fn change_context(&self) -> Option<&ChangeContextClosure> {
        match self.change_context {
            Some(ref change_context_closure) => Some(change_context_closure),
            None => None
        }
    }
    pub fn set_next(&mut self, next: NextClosure) {
        self.next = Some(next);
    }
    pub fn set_change_context(&mut self, change_context: ChangeContextClosure) {
        self.change_context = Some(change_context);
    }
}

#[cfg(test)]
mod story_link_tests {
    use crate::story::context::ContextValue;

    use super::*;

    #[test]
    fn next_works() {
        let mut story_link = StoryLink::new();
        let mut story_context = Context::new();
        story_context.insert(String::from("count"), ContextValue::Integer(0));

        story_link.set_next(|context| {
            match context.get("count").unwrap() {
                ContextValue::Integer(count) => {
                    if *count < 5 {
                        String::from("first-story-node")
                    } else {
                        String::from("second-story-node")
                    }
                }
                _ => panic!("missing count")
            }
        });

        assert_eq!(story_link.next().unwrap()(&mut story_context), "first-story-node");
        *story_context.get_mut("count").unwrap() = ContextValue::Integer(6);
        assert_eq!(story_link.next().unwrap()(&mut story_context), "second-story-node");
    }

    #[test]
    fn change_context_works() {
        let mut story_link = StoryLink::new();
        let mut story_context = Context::new();
        story_context.insert(String::from("count"), ContextValue::Integer(0));

        story_link.set_change_context(|c| {
            match c.get_mut("count").unwrap() {
                ContextValue::Integer(count) => *count += 1,
                _ => panic!("missing count")
            }
        });

        story_link.change_context().unwrap()(&mut story_context);
        assert_eq!(*story_context.get("count").unwrap(), ContextValue::Integer(1));
    }
}
