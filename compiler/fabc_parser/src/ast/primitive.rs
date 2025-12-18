#[derive(Debug, PartialEq)]
pub enum Primitive {
    Identifier(String),
    Path(Vec<String>),
}
