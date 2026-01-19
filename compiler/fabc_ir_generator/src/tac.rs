pub enum Value {
    String(String),
    Number(i64),
    Boolean(bool),
}

pub enum Address {
    Name(String),
    Constant(Value),
    Temporary(String),
}
