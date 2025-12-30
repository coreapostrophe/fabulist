use std::collections::HashMap;

use fabc_parser::Parsable;

use crate::{error::Error, reachability::Reachability};

pub mod error;
pub mod implementations;
pub mod reachability;

pub trait Analyzable {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error>;
}

pub struct Analyzer {
    reachability_map: HashMap<usize, Reachability>,
}

impl Analyzer {
    pub fn analyze<T>(ast: &T) -> Result<Self, Error>
    where
        T: Parsable + Analyzable,
    {
        let mut analyzer = Self {
            reachability_map: HashMap::new(),
        };
        ast.analyze(&mut analyzer)?;
        Ok(analyzer)
    }
    pub fn get_reachability(&self, node_id: &usize) -> Option<&Reachability> {
        self.reachability_map.get(node_id)
    }
    pub(crate) fn override_reachability(&mut self, node_id: usize, reachability: Reachability) {
        self.reachability_map.insert(node_id, reachability);
    }
    pub(crate) fn set_reachability_if_absent(
        &mut self,
        node_id: usize,
        reachability: Reachability,
    ) {
        self.reachability_map.entry(node_id).or_insert(reachability);
    }
}
