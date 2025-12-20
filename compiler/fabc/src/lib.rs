use crate::error::Error;

pub mod error;

pub struct Compiler<'a> {
    pub entry: &'a str,
    pub modules: Vec<&'a str>,
}

impl<'a> Compiler<'a> {
    pub fn compile(entry: &'a str) -> Result<(), Error> {
        let compiler = Compiler {
            entry,
            modules: Vec::new(),
        };

        compiler.run()
    }

    pub fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}
