use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Fetch highlights from GoodLinks and write to source directory
    Import {
        /// Directory to write fetched highlight files into
        #[arg(short, long)]
        source: PathBuf,

        /// Dry run - show what would be written without writing
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Copy new highlights from source directory to target directory
    Sync {
        /// Source directory containing Markdown highlight files
        #[arg(short, long)]
        source: PathBuf,

        /// Target directory to copy new files into
        #[arg(short, long)]
        target: PathBuf,

        /// Dry run - show what would be copied without copying
        #[arg(short, long)]
        dry_run: bool,
    },
}
