use clap::Parser;

use crate::cli::Opts;
use crate::cmd::CommandError;

pub mod cli;
pub mod cmd;
pub mod common;
pub mod core;

fn main() -> Result<(), CommandError> {
    let opts: Opts = Opts::parse();

    match opts {
        Opts::Run {
            path,
            trace,
            output,
        } => cmd::run(path, trace, output),
    }
}
