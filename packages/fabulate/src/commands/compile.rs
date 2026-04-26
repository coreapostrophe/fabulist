use std::path::PathBuf;

use fabc::{CompileOptions, Compiler};

use crate::error::Result;

#[derive(clap::Args)]
pub struct Compile {
    /// The input fab source file
    pub input: PathBuf,

    /// Output path for the generated LLVM IR
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Optional output directory for a compiled bundle containing LLVM IR and story metadata
    #[arg(long = "bundle")]
    pub bundle_output: Option<PathBuf>,

    /// Override the emitted LLVM module name
    #[arg(long)]
    pub module_name: Option<String>,
}

impl Compile {
    pub fn exec(&self) -> Result<()> {
        let artifact = Compiler::compile_with_options(CompileOptions {
            entry: self.input.clone(),
            output: self.output.clone(),
            module_name: self.module_name.clone(),
            bundle_output: self.bundle_output.clone(),
        })?;

        println!(
            "Wrote LLVM IR for {} to {}",
            artifact.entry.display(),
            artifact.output_path.display()
        );

        if let Some(bundle) = artifact.bundle {
            println!(
                "Wrote compiled bundle manifest to {}",
                bundle.manifest_path.display()
            );
        }

        Ok(())
    }
}
