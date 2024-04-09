use std::borrow::BorrowMut;

use crate::{
    error::Result,
    state::State,
    story::{traits::Progressive, DialogueIndex, Story},
};

pub struct Engine<Str, Stt>
where
    Str: BorrowMut<Story>,
    Stt: BorrowMut<State>,
{
    story: Str,
    state: Stt,
}

impl<Str, Stt> Engine<Str, Stt>
where
    Str: BorrowMut<Story>,
    Stt: BorrowMut<State>,
{
    pub fn new(story: Str, state: Stt) -> Self {
        Self { story, state }
    }
    pub fn state(&self) -> &Stt {
        &self.state
    }
    pub fn mut_state(&mut self) -> &mut Stt {
        &mut self.state
    }
    pub fn story(&self) -> &Str {
        &self.story
    }
    pub fn mut_story(&mut self) -> &mut Str {
        &mut self.story
    }
    pub fn start(&mut self) -> Result<DialogueIndex> {
        self.state
            .borrow_mut()
            .set_current_part(self.story.borrow().start());
        self.next(None)
    }
    pub fn next(&mut self, choice_index: Option<usize>) -> Result<DialogueIndex> {
        Story::next(
            self.story.borrow_mut(),
            self.state.borrow_mut(),
            choice_index,
        )
    }
}
