#[derive(Clone)]
pub struct Field {
    pub name: String,
    pub r#type: DataType,
}

#[derive(Clone)]
pub enum DataType {
    Number,
    Boolean,
    String,
    None,
    Record { fields: Vec<Field> },
}
