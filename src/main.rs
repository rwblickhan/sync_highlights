mod cli;
mod import;
mod models;
mod sync;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Import { source, dry_run } => import::import(&source, dry_run),
        Commands::Sync {
            source,
            target,
            dry_run,
        } => sync::sync(&source, &target, dry_run),
    }
}
