use crate::error::Error;

pub mod error;

pub struct Compiler<'a> {
    pub entry: &'a str,
}

impl<'a> Compiler<'a> {
    pub fn compile(entry: &'a str) -> Result<(), Error> {
        let compiler = Compiler { entry };

        compiler.run()
    }

    pub fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}
