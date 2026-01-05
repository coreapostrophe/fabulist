use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone)]
pub enum SymbolType {
    Variable,
    Function,
    Struct,
    Enum,
    Trait,
}

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub r#type: SymbolType,
    pub level: Option<usize>,
    pub var: Option<usize>,
    pub depth: usize,
}

#[derive(Clone)]
pub enum Entry {
    Value {
        value: Symbol,
        psl: usize,
        hash: u64,
    },
    Available,
}

impl Entry {
    pub fn set_psl(&mut self, psl: usize) {
        if let Entry::Value { psl: entry_psl, .. } = self {
            *entry_psl = psl;
        }
    }
}

#[allow(unused)]
pub struct SymbolTable {
    size: usize,
    entries: Vec<Entry>,
    scope_display: Vec<Option<usize>>,
    current_depth: usize,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new(16)
    }
}

#[allow(unused)]
impl SymbolTable {
    pub fn new(capacity: usize) -> Self {
        Self {
            size: 0,
            entries: vec![Entry::Available; capacity],
            scope_display: vec![None],
            current_depth: 0,
        }
    }
    fn make_hash(name: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        hasher.finish()
    }
    fn push_entry(&mut self, symbol_info: Symbol) {
        let hash = Self::make_hash(&symbol_info.name);

        let mut entry = Entry::Value {
            value: symbol_info,
            psl: 0,
            hash,
        };

        if self.size + 1 > (self.entries.len() * 3) / 4 {
            let new_cap = self.entries.len() * 2;
            self.resize_and_rehash(new_cap);
        }

        let mut idx = (hash % self.entries.len() as u64) as usize;
        let mut probe_dist = 0;

        while let Entry::Value { psl, .. } = &self.entries[idx] {
            if *psl < probe_dist {
                entry.set_psl(probe_dist);
                probe_dist = *psl;
                std::mem::swap(&mut self.entries[idx], &mut entry);
            }
            idx = (idx + 1) % self.entries.len();
            probe_dist += 1;
        }

        self.entries[idx] = entry;
        self.size += 1;
    }
    fn get_entry(&self, name: &str) -> Option<&Symbol> {
        let hash = Self::make_hash(name);
        let mut idx = (hash % self.entries.len() as u64) as usize;
        let mut probe_dist = 0;

        while let Entry::Value {
            value,
            psl,
            hash: entry_hash,
        } = &self.entries[idx]
        {
            if *entry_hash == hash && value.name == name {
                return Some(value);
            } else if *psl < probe_dist {
                return None;
            } else {
                idx = (idx + 1) % self.entries.len();
                probe_dist += 1;
            }
        }

        None
    }
    fn remove_entry(&mut self, name: &str) -> Option<Symbol> {
        let hash = Self::make_hash(name);
        let entries_len = self.entries.len();
        let mut idx = (hash % entries_len as u64) as usize;
        let mut probe_dist = 0;

        while let Entry::Value {
            value,
            psl,
            hash: entry_hash,
        } = &self.entries[idx]
        {
            if *entry_hash == hash && value.name == name {
                let removed_entry = std::mem::replace(&mut self.entries[idx], Entry::Available);
                if let Entry::Value { value, .. } = removed_entry {
                    self.size -= 1;

                    while let Entry::Value { psl: next_psl, .. } =
                        &mut self.entries[(idx + 1) % entries_len]
                    {
                        if *next_psl > 0 {
                            *next_psl -= 1;
                            let next_idx = (idx + 1) % entries_len;
                            self.entries[idx] =
                                std::mem::replace(&mut self.entries[next_idx], Entry::Available);
                            idx = next_idx;
                        } else {
                            break;
                        }
                    }

                    return Some(value);
                }
            } else if *psl < probe_dist {
                return None;
            } else {
                idx = (idx + 1) % entries_len;
                probe_dist += 1;
            }
        }

        None
    }
    fn resize_and_rehash(&mut self, new_capacity: usize) {
        if new_capacity == 0 {
            return;
        }

        let old_entries =
            std::mem::replace(&mut self.entries, vec![Entry::Available; new_capacity]);
        self.size = 0;

        for entry in old_entries.into_iter() {
            if let Entry::Value { value, .. } = entry {
                self.push_entry(value);
            }
        }
    }
}
