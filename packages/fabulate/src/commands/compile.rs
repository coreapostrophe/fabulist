use std::path::PathBuf;

#[derive(clap::Args)]
pub struct Compile {
    /// The input fab source file
    pub input: PathBuf,
}

impl Compile {
    pub fn exec(&self) {
        println!("Compiling file: {}", self.input.display());
    }
}
