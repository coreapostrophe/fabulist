use crate::{
    error::Result,
    state::State,
    story::{traits::Progressive, DialogueIndex, Story},
};

pub struct Engine {
    state: State,
    story: Story,
}

impl Engine {
    pub fn new(story: impl Into<Story>, state: impl Into<State>) -> Self {
        Self {
            story: story.into(),
            state: state.into(),
        }
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn mut_state(&mut self) -> &mut State {
        &mut self.state
    }
    pub fn story(&self) -> &Story {
        &self.story
    }
    pub fn mut_story(&mut self) -> &mut Story {
        &mut self.story
    }
    pub fn start(&mut self) -> Result<DialogueIndex> {
        self.state.set_current_part(self.story.start());
        self.next(None)
    }
    pub fn next(&mut self, choice_index: Option<usize>) -> Result<DialogueIndex> {
        self.story.next(&mut self.state, choice_index)
    }
}
