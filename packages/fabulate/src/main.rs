use clap::Parser;

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

fn main() {
    let cli = Cli::parse();
    cli.command.exec();
}
