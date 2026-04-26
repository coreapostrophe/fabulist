use clap::Parser;
use std::process::ExitCode;

use crate::commands::Commands;

mod commands;
mod error;

/// Fabulist compiler cli
#[derive(clap::Parser)]
#[command(version, about)]
struct Cli {
    /// The command to run
    #[clap(subcommand)]
    command: Commands,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command.exec() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
