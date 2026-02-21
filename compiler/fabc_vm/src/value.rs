use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(Rc<String>),
    Number(i64),
    Boolean(bool),
    Address(usize),
    None,
}
