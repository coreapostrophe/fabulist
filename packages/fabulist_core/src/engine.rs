use std::{borrow::BorrowMut, fmt::Debug};

use crate::{
    error::EngineResult,
    state::State,
    story::{reference::DialogueIndex, Story},
};

pub trait Progressive: Debug {
    type Output;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output;
}

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
    pub fn start(&mut self) -> EngineResult<DialogueIndex> {
        let start_key = self.story.borrow().start().cloned();
        self.state.borrow_mut().set_current_part(start_key);
        self.next(None)
    }
    pub fn next(&mut self, choice_index: Option<usize>) -> EngineResult<DialogueIndex> {
        Story::next(
            self.story.borrow_mut(),
            self.state.borrow_mut(),
            choice_index,
        )
    }
}
