use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    String(Rc<String>),
    Number(f64),
    Boolean(bool),
    None,
}
