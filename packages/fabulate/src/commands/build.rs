use std::path::PathBuf;

use fabc::{Compiler, ExecutableOptions};

use crate::error::Result;

#[derive(clap::Args)]
pub struct Build {
    /// The input fab source file
    pub input: PathBuf,

    /// Output path for the generated standalone executable
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Override the emitted module name used for the standalone build
    #[arg(long)]
    pub module_name: Option<String>,

    /// Build the generated launcher in release mode
    #[arg(long)]
    pub release: bool,
}

impl Build {
    pub fn exec(&self) -> Result<()> {
        let artifact = Compiler::build_executable_with_options(ExecutableOptions {
            entry: self.input.clone(),
            output: self.output.clone(),
            module_name: self.module_name.clone(),
            release: self.release,
        })?;

        println!(
            "Wrote standalone executable for {} to {}",
            artifact.entry.display(),
            artifact.output_path.display()
        );

        Ok(())
    }
}
