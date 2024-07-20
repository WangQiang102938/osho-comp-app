use clap::Parser;
use cli::*;

mod cli;
mod shared_utils;


fn main() {
    let mut opts=CliOpts::parse();
    println!("{:?}",opts);
}
