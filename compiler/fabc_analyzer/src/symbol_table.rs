use std::collections::HashMap;

use crate::types::Symbol;

pub struct SymbolTable<T> {
    entries: HashMap<String, Vec<Symbol<T>>>,
    scope_display: Vec<Vec<String>>,
    current_level: usize,
    scope_slots: Vec<usize>,
}

impl<T> Default for SymbolTable<T> {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            scope_display: vec![Vec::new()],
            current_level: 0,
            scope_slots: vec![0],
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
        self.scope_slots.push(0);
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
        self.scope_slots.pop();
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
        let slot = self
            .scope_slots
            .last_mut()
            .map(|counter| {
                let current = *counter;
                *counter += 1;
                current
            })
            .unwrap_or(0);

        let symbol = Symbol {
            name: name.to_string(),
            r#type,
            slot,
            depth: self.current_level,
        };

        self.insert_symbol(symbol)
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol<T>> {
        self.entries.get(name).and_then(|stack| stack.last())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_and_lookup_symbol_in_current_scope() {
        let mut table = SymbolTable::new();
        table.assign_symbol("x", 1);
        let sym = table.lookup_symbol("x").expect("symbol missing");
        assert_eq!(sym.name, "x");
        assert_eq!(sym.r#type, 1);
    }

    #[test]
    fn respects_scopes_and_pops_on_exit() {
        let mut table = SymbolTable::new();
        table.assign_symbol("x", 1);
        table.enter_scope();
        table.assign_symbol("x", 2);
        assert_eq!(table.lookup_symbol("x").unwrap().r#type, 2);
        table.exit_scope();
        assert_eq!(table.lookup_symbol("x").unwrap().r#type, 1);
    }
}
