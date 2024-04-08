use super::context::Context;

pub type QueryNextClosure = fn(&Context) -> String;
pub type ChangeContextClosure = fn(&mut Context) -> ();
