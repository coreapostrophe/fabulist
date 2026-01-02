use std::collections::HashMap;

use fabc_error::Error;
use fabc_parser::Parsable;

use crate::reachability::Reachability;

pub mod implementations;
pub mod reachability;

pub trait Analyzable {
    fn analyze(&self, analyzer: &mut Analyzer);
}

pub struct Analyzer {
    reachability_map: HashMap<usize, Reachability>,
    errors: Vec<Error>,
}

impl Analyzer {
    pub fn analyze<T>(ast: &T) -> Result<Self, Error>
    where
        T: Parsable + Analyzable,
    {
        let mut analyzer = Self {
            reachability_map: HashMap::new(),
            errors: Vec::new(),
        };
        ast.analyze(&mut analyzer);
        Ok(analyzer)
    }
    pub fn get_reachability(&self, node_id: usize) -> Option<&Reachability> {
        self.reachability_map.get(&node_id)
    }
    pub(crate) fn set_reachability(&mut self, node_id: usize) -> &mut Reachability {
        self.reachability_map.entry(node_id).or_default()
    }
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
    pub(crate) fn _push_error(&mut self, error: Error) {
        self.errors.push(error);
    }
}
