use crate::commands::compile::Compile;

pub mod compile;

#[derive(clap::Subcommand)]
pub enum Commands {
    Compile(Compile),
}

impl Commands {
    pub fn exec(&self) {
        match self {
            Commands::Compile(cmd) => {
                cmd.exec();
            }
        }
    }
}
