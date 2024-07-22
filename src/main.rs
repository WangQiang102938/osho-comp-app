use clap::Parser;
use cli::*;

mod cli;
mod shared_utils;
mod archiver;
mod session;

fn main() {
    let mut opts=CliOpts::parse();
    println!("{:?}",opts);
}
