use self::engine_error::{EngineError, EngineErrorType};
use super::story::Story;

pub mod engine_error;

pub type EngineResult = Result<String, EngineError>;

pub struct Engine {
    story: Option<Story>
}

impl Engine {
    pub fn new(story: Option<Story>) -> Self {
        Self { story }
    }
    
    pub fn story(&self) -> Option<&Story> {
        match self.story {
            Some(ref story) => Some(story),
            None => None
        }
    }
    pub fn story_mut(&mut self) -> Option<&mut Story> {
        match &mut self.story {
            Some(ref mut story) => Some(story),
            None => None
        }
    }
    
    pub fn start(&mut self) -> EngineResult {
        let story = self.story_mut().ok_or(EngineError::new(EngineErrorType::NoStory))?;
        {
            let start_node = story.start();
            story.set_current(start_node.clone());
        }
        let start_node = story.start();
        Ok(start_node.clone())
    }
}