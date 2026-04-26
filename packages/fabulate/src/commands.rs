use crate::commands::build::Build;
use crate::commands::compile::Compile;
use crate::commands::play::Play;
use crate::error::Result;

pub mod build;
pub mod compile;
pub mod play;

#[derive(clap::Subcommand)]
pub enum Commands {
    Build(Build),
    Compile(Compile),
    Play(Play),
}

impl Commands {
    pub fn exec(&self) -> Result<()> {
        match self {
            Commands::Build(cmd) => cmd.exec(),
            Commands::Compile(cmd) => cmd.exec(),
            Commands::Play(cmd) => cmd.exec(),
        }
    }
}
