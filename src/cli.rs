use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Target directory to copy new files into
    #[arg(short, long)]
    pub target: PathBuf,

    /// Dry run - show what would be copied without copying
    #[arg(short, long)]
    pub dry_run: bool,

    /// Print verbose output including import progress
    #[arg(short, long)]
    pub verbose: bool,
}
