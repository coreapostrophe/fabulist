#[derive(Debug, PartialEq)]
pub enum Literal {
    Boolean(bool),
    String(String),
    Number(f64),
    None,
}
