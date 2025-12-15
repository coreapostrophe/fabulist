use crate::{
    error::EngineResult,
    story::{context::Contextual, reference::ListKey},
};

pub type QueryNextClosure = Box<dyn Fn(&dyn Contextual) -> EngineResult<ListKey<String>>>;
pub trait QueryNext {
    fn query_next(&self) -> Option<&QueryNextClosure>;
    fn set_query_next(&mut self, closure: QueryNextClosure);
}

pub type ChangeContextClosure = Box<dyn Fn(&mut dyn Contextual) -> EngineResult<()>>;
pub trait ChangeContext {
    fn change_context(&self) -> Option<&ChangeContextClosure>;
    fn set_change_context(&mut self, closure: ChangeContextClosure);
}
