use std::collections::HashMap;

pub enum SymbolType {
    Variable,
    Function,
    Struct,
    Enum,
    Trait,
}

pub struct Symbol {
    r#type: SymbolType,
    scope_level: usize,
}

impl Symbol {
    pub fn new(r#type: SymbolType, scope_level: usize) -> Self {
        Self {
            r#type,
            scope_level,
        }
    }
    pub fn r#type(&self) -> &SymbolType {
        &self.r#type
    }
    pub fn scope_level(&self) -> usize {
        self.scope_level
    }
}

#[derive(Default)]
pub struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert_symbol(&mut self, name: String, symbol: Symbol) {
        self.symbols.insert(name, symbol);
    }
    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

#[derive(Default)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }
    fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }
    fn exit_scope(&mut self) {
        self.scopes.pop();
    }
    pub fn handle_scope<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.enter_scope();
        let result = f(self);
        self.exit_scope();
        result
    }
    pub fn insert_symbol(&mut self, name: String, symbol: Symbol) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert_symbol(name, symbol);
        }
    }
    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get_symbol(name) {
                return Some(symbol);
            }
        }
        None
    }
}
