use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    String(Rc<String>),
    Number(i64),
    Boolean(bool),
    None,
}
