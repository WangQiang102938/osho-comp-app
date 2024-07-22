use clap::Parser;
use cli::*;
use task::ArchiveTask;

mod cli;
mod shared_utils;
mod archiver;
mod task;

fn main() {
    let mut opts=CliOpts::parse();
    let mut task=ArchiveTask::parse_opt(&mut opts);
    println!("{:?}",opts);
    println!("{:?}",task);
}
