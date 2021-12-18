use clap::Parser;

use crate::cli::Opts;

pub mod cli;
pub mod common;

fn main() {
    let _opts: Opts = Opts::parse(); /* TODO: remove underscore */

    println!("Hello, world!");
}
