use fabc_error::Error;
use fabc_parser::Parsable;

pub mod implementations;
pub mod reachability;
pub mod symbol_table;

pub trait Analyzable {
    fn analyze(&self, analyzer: &mut Analyzer);
}

pub struct Analyzer {
    errors: Vec<Error>,
}

impl Analyzer {
    pub fn analyze<T>(ast: &T) -> Result<Self, Error>
    where
        T: Parsable + Analyzable,
    {
        let mut analyzer = Self { errors: Vec::new() };
        ast.analyze(&mut analyzer);
        Ok(analyzer)
    }
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
    pub(crate) fn _push_error(&mut self, error: Error) {
        self.errors.push(error);
    }
}
