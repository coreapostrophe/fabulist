use std::collections::HashMap;

use crate::types::Symbol;

pub struct SymbolTable<T> {
    entries: HashMap<String, Vec<Symbol<T>>>,
    scope_display: Vec<Vec<String>>,
    current_level: usize,
}

impl<T> Default for SymbolTable<T> {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            scope_display: vec![Vec::new()],
            current_level: 0,
        }
    }
}

impl<T> SymbolTable<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enter_scope(&mut self) {
        self.current_level += 1;
        self.scope_display.push(Vec::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scope_display.len() == 1 {
            return;
        }
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

    fn insert_symbol(&mut self, symbol: Symbol<T>) -> Option<&Symbol<T>> {
        let sym_key = symbol.name.clone();

        let entry = self.entries.entry(sym_key.clone()).or_default();

        entry.push(symbol);

        if let Some(scope_symbols) = self.scope_display.last_mut() {
            scope_symbols.push(sym_key);
        }

        entry.last()
    }

    pub fn assign_symbol(&mut self, name: &str, r#type: T) -> Option<&Symbol<T>> {
        let symbol = Symbol {
            name: name.to_string(),
            r#type,
        };

        self.insert_symbol(symbol)
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol<T>> {
        self.entries.get(name).and_then(|stack| stack.last())
    }
}
