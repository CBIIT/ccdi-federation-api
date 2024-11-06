//! Curates definitions of certain CDEs programmatically.

use std::io::IsTerminal as _;

use clap::Parser;
use clap::Subcommand;
use clap_verbosity_flag::Verbosity;
use eyre::Result;
use tracing_log::AsTrace as _;

pub(crate) mod http;
mod uberon;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Curates the list of Uberon codes as a Rust enum.
    Uberon(uberon::Args),
}

/// A tool to execute the curation of CCDI CDEs from public standards.
#[derive(Debug, Parser)]
pub struct Args {
    /// The subcommand to execute.
    #[clap(subcommand)]
    command: Command,

    /// The verbosity flags.
    #[command(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<()> {
    let args = Args::parse();

    color_eyre::install()?;
    tracing_log::LogTracer::init()?;

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .with_writer(std::io::stderr)
        .with_ansi(std::io::stderr().is_terminal())
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    match args.command {
        Command::Uberon(args) => uberon::main(args),
    }
}
