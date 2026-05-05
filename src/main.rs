mod cli;
mod import;
mod models;
mod sync;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let tmp = tempfile::tempdir()?;
    import::import(tmp.path(), cli.dry_run, cli.verbose)?;
    sync::sync(tmp.path(), &cli.target, cli.dry_run, cli.verbose)
}
