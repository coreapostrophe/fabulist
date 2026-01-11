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

#[derive(Clone)]
pub enum ModuleSymbolType {
    Data(DataType),
    Function {
        return_type: DataType,
        parameters: Vec<DataType>,
    },
}

#[derive(Clone)]
pub enum StorySymbolType {
    Part,
    Speaker,
}

#[derive(Clone)]
pub struct Symbol<T> {
    pub name: String,
    pub r#type: T,
}
