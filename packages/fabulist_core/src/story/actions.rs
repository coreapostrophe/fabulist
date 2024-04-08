use super::context::Context;

pub type NextClosure = fn(&Context) -> String;
pub type ChangeContextClosure = fn(&mut Context) -> ();
