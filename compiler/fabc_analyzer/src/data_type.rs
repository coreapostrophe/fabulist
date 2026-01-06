pub struct Field {
    pub name: String,
    pub r#type: DataType,
}

pub enum DataType {
    Number,
    Boolean,
    String,
    Record { fields: Vec<Field> },
}
