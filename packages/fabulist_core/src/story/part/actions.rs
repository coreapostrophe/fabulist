use crate::story::context::Context;

pub type QueryNextClosure = fn(&Context) -> String;
pub trait QueryNext {
    fn query_next(&self) -> Option<&QueryNextClosure>;
    fn set_query_next(&mut self, closure: QueryNextClosure);
}

pub type ChangeContextClosure = fn(&mut Context) -> ();
pub trait ChangeContext {
    fn change_context(&self) -> Option<&ChangeContextClosure>;
    fn set_change_context(&mut self, closure: ChangeContextClosure);
}
