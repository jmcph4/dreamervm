use clap::Parser;

pub mod common;

#[derive(Clone, Debug, Parser)]
#[clap(about, version, author)]
pub struct Opts {/* TODO: add command-line options here! */}

fn main() {
    let _opts: Opts = Opts::parse(); /* TODO: remove underscore */

    println!("Hello, world!");
}
