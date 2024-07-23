use clap::Parser;
use cli::*;
use task::ArchiveTask;

mod cli;
mod shared_utils;
mod archiver;
mod task;

fn main() {
    let mut opts=CliOpts::parse();
    let mut task=ArchiveTask::parse_opt(&mut opts).expect("Task creation failed.");
    task.exec().expect("Task execute failed.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use cli::RunMode;
    use std::path::PathBuf;

    #[test]
    fn test_run() {
        let mut opts = CliOpts::parse_from(&[
            "ozipw",
            "-m",
            "Auto",
            "-d",
            "test.zip",
            "src",
            "LICENSE",
            "README.md",
            "Cargo.toml",
        ]);
        assert_eq!(opts.mode, RunMode::Auto);
        assert_eq!(opts.dest, Some(PathBuf::from("test.zip")));
        assert_eq!(opts.args.len(), 4);
        assert_eq!(
            opts.args,
            vec![
                PathBuf::from("src"),
                PathBuf::from("LICENSE"),
                PathBuf::from("README.md"),
                PathBuf::from("Cargo.toml")
            ]
        );

        let mut task=ArchiveTask::parse_opt(&mut opts).expect("Task creation failed.");
        task.exec().expect("Task execute failed.");

    }
}
