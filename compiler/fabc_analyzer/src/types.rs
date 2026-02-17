use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub struct Field {
    pub name: String,
    pub r#type: Box<ModuleSymbolType>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DataType {
    Number,
    Boolean,
    String,
    None,
    Context,
    Record { fields: Vec<Field> },
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Number => write!(f, "Number"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::String => write!(f, "String"),
            DataType::None => write!(f, "None"),
            DataType::Context => write!(f, "Context"),
            DataType::Record { fields } => {
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|field| format!("{}: {}", field.name, field.r#type))
                    .collect();
                write!(f, "Record {{ {} }}", field_strs.join(", "))
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ModuleSymbolType {
    Data(DataType),
    Module {
        name: String,
    },
    Function {
        return_type: Box<ModuleSymbolType>,
        parameters: Vec<ModuleSymbolType>,
        arity: usize,
    },
}

impl Display for ModuleSymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleSymbolType::Module { name } => write!(f, "module {name}"),
            ModuleSymbolType::Data(data_type) => write!(f, "{data_type}"),
            ModuleSymbolType::Function {
                return_type,
                parameters,
                ..
            } => {
                let params: Vec<String> = parameters.iter().map(|p| format!("{p}")).collect();
                write!(f, "fn({}) -> {}", params.join(", "), return_type)
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum StorySymbolType {
    Part,
    Speaker,
}

impl Display for StorySymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorySymbolType::Part => write!(f, "part"),
            StorySymbolType::Speaker => write!(f, "speaker"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Symbol<T> {
    pub name: String,
    pub r#type: T,
}

#[derive(Debug)]
pub struct SymbolAnnotation<T> {
    pub name: Option<String>,
    pub r#type: T,
}

impl<T> From<Symbol<T>> for SymbolAnnotation<T> {
    fn from(symbol: Symbol<T>) -> Self {
        SymbolAnnotation {
            name: Some(symbol.name),
            r#type: symbol.r#type,
        }
    }
}
