use std::collections::HashMap;

use crate::data_type::DataType;

pub enum SymbolType {
    Variable(DataType),
    Function {
        return_type: DataType,
        parameters: Vec<DataType>,
    },
    Part,
    Speaker,
}

pub struct Symbol {
    pub name: String,
    pub r#type: SymbolType,
    pub scope_level: usize,
}

pub struct SymbolTable {
    entries: HashMap<String, Vec<Symbol>>,
    scope_display: Vec<Vec<String>>,
    current_level: usize,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            scope_display: vec![Vec::new()],
            current_level: 0,
        }
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn enter_scope(&mut self) {
        self.current_level += 1;
        self.scope_display.push(Vec::new());
    }
    pub fn exit_scope(&mut self) {
        if let Some(scope_symbols) = self.scope_display.pop() {
            for symbol_key in scope_symbols {
                if let Some(symbol_stack) = self.entries.get_mut(&symbol_key) {
                    symbol_stack.pop();
                    if symbol_stack.is_empty() {
                        self.entries.remove(&symbol_key);
                    }
                }
            }
        }
        if self.current_level > 0 {
            self.current_level -= 1;
        }
    }
    pub fn insert_symbol(&mut self, name: &str, r#type: SymbolType) {
        let symbol = Symbol {
            name: name.to_string(),
            r#type,
            scope_level: self.current_level,
        };

        self.entries
            .entry(name.to_string())
            .or_default()
            .push(symbol);

        if let Some(scope_symbols) = self.scope_display.last_mut() {
            scope_symbols.push(name.to_string());
        }
    }
    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.entries.get(name).and_then(|stack| stack.last())
    }
}
