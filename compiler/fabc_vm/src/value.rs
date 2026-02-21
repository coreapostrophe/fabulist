use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    String(Rc<String>),
    Number(i64),
    Boolean(bool),
    Address(usize),
    None,
}
