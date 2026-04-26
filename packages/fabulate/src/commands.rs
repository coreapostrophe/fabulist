use crate::commands::compile::Compile;
use crate::commands::play::Play;
use crate::error::Result;

pub mod compile;
pub mod play;

#[derive(clap::Subcommand)]
pub enum Commands {
    Compile(Compile),
    Play(Play),
}

impl Commands {
    pub fn exec(&self) -> Result<()> {
        match self {
            Commands::Compile(cmd) => cmd.exec(),
            Commands::Play(cmd) => cmd.exec(),
        }
    }
}
