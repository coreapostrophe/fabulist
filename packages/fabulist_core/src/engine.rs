use crate::{
    error::Result,
    state::{DialogueIndex, State},
    story::{traits::Progressive, Story},
};

pub struct Engine {
    state: State,
    story: Story,
}

impl Engine {
    pub fn new(story: Story, state: State) -> Self {
        Self { state, story }
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn story(&self) -> &Story {
        &self.story
    }
    pub fn start(&mut self) -> Result<DialogueIndex> {
        self.state.set_current_part(self.story.start());
        self.next(None)
    }
    pub fn next(&mut self, choice_index: Option<usize>) -> Result<DialogueIndex> {
        self.story.next(&mut self.state, choice_index)
    }
}
