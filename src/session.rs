use crate::archiver::{Archiver, ArchiverMode};
use crate::shared_utils::{AppError, AppResult};
use crate::{CliOpts, RunMode};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ArchiveTask {
    jobs: Vec<ArchiveJob>,
    wizard_enable: bool,
    options: HashMap<String, String>,
}

impl ArchiveTask {
    fn new() -> ArchiveTask {
        return ArchiveTask {
            jobs: Vec::new(),
            wizard_enable: false,
            options: HashMap::new(),
        };
    }

    fn parse_opt(opt: &mut CliOpts)->AppResult<ArchiveTask> {
        let mut session = ArchiveTask::new();
        let runmode_res = opt.check_runmode();
        if runmode_res.is_err() {
            return Err(AppError::Unknown("RunMode not support".to_string()));
        }
        session.wizard_enable = runmode_res.unwrap() == RunMode::Wizard;
        // TODO: App Options
        return Ok(session);
    }
}

pub struct ArchiveJob {
    source_paths: Vec<PathBuf>,
    target_path: PathBuf,
    archiver: Box<dyn Archiver>,
    mode: ArchiverMode,
    overwrite: bool,
    with_creation: bool,
    options: HashMap<String, String>,
}

impl ArchiveJob {
    fn new() -> ArchiveJob {
        return ArchiveJob {
            source_paths: Vec::new(),
            target_path: PathBuf::new(),
            archiver: todo!("Implement later: Archiver"),
            mode: ArchiverMode::Unknown,
            overwrite: false,
            with_creation: false,
            options: HashMap::new(),
        };
    }

    fn parse_opt(opt: &mut CliOpts)->AppResult<ArchiveTask>{
        todo!()
    }
}

pub fn opt2session(opt: &mut CliOpts) -> AppResult<ArchiveTask> {
    todo!();
}
