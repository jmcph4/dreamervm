use std::path::PathBuf;

use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(about, version, author)]
pub enum Opts {
    #[clap(override_help = "Executes a Dreamer program")]
    Run {
        path: PathBuf,
        #[clap(long, short)]
        trace: bool,
        output: Option<PathBuf>,
    },
}
