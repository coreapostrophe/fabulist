use crate::symbol_table::SymbolType;

pub struct SymbolAnnotation {
    pub r#type: SymbolType,
    pub scope_level: usize,
}
