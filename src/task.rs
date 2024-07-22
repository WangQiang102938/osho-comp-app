use crate::archiver::{Archiver, ArchiverMode, DummyArchiver};
use crate::shared_utils::{get_supported_format, AppError, AppResult};
use crate::{CliOpts, RunMode};
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ArchiveTask {
    jobs: Vec<ArchiveJob>,
    wizard_enable: bool,
    options: HashMap<String, String>,
}

impl ArchiveTask {
    pub fn new() -> ArchiveTask {
        return ArchiveTask {
            jobs: Vec::new(),
            wizard_enable: false,
            options: HashMap::new(),
        };
    }

    pub fn parse_opt(opt: &mut CliOpts) -> AppResult<ArchiveTask> {
        let mut tmp_task = ArchiveTask::new();
        tmp_task.wizard_enable = opt.mode == RunMode::Wizard;
        // TODO: App Options
        let tmp_job = ArchiveJob::parse_opt(opt);
        match tmp_job {
            Ok(job) => tmp_task.jobs.push(job),
            Err(e) => return Err(e),
        }
        return Ok(tmp_task);
    }
}

#[derive(Debug)]
pub struct ArchiveJob {
    pub source_paths: Vec<PathBuf>,
    pub target_path: PathBuf,
    pub archiver: Box<dyn Archiver>,
    pub mode: ArchiverMode,
    pub overwrite: bool,
    pub with_creation: bool,
    pub options: HashMap<String, String>,
}

impl ArchiveJob {
    fn new() -> ArchiveJob {
        return ArchiveJob {
            source_paths: Vec::new(),
            target_path: env::current_dir().expect("Failed to get current dir."),
            archiver: Box::new(DummyArchiver{}),
            mode: ArchiverMode::Unknown,
            overwrite: false,
            with_creation: false,
            options: HashMap::new(),
        };
    }

    fn parse_opt(opt: &mut CliOpts) -> AppResult<ArchiveJob> {
        let mut tmpjob = ArchiveJob::new();
        tmpjob.source_paths = opt.args.clone();
        tmpjob.target_path = opt.dest.clone().unwrap_or(env::current_dir().expect("Failed to get curr dir"));
        tmpjob.overwrite = opt.overwrite.clone();

        if opt.mode == RunMode::Auto {
            if ArchiveJob::is_all_extractable(&tmpjob.source_paths) {
                tmpjob.mode = ArchiverMode::Extract;
            } else {
                tmpjob.mode = ArchiverMode::Archive;
            }
        }

        return Ok(tmpjob);
    }

    fn is_all_extractable(paths: &Vec<PathBuf>) -> bool {
        return paths.iter().all(|arg| {
            let path = arg.to_str().unwrap().to_lowercase();
            let exts = get_supported_format(true);
            for ext in exts.iter() {
                if path.ends_with(ext) {
                    return true;
                }
            }
            return false;
        });
    }

    fn is_all_extractable_s(&self) -> bool {
        return ArchiveJob::is_all_extractable(&self.source_paths);
    }

    fn job_selfcheck(&self) -> AppResult<bool> {
        for path in self.source_paths.iter() {
            if !path.exists() {
                return Err(AppError::TaskError(
                    format!("Job sourcefile not exist: {:?}", path).to_string(),
                ));
            }
        }

        match self.mode {
            ArchiverMode::Unknown => {
                return Err(AppError::TaskError("Job mode is unknown".to_string()))
            }
            ArchiverMode::Extract => {
                if !self.is_all_extractable_s() {
                    return Err(AppError::TaskError(
                        "Job mode is extract but some source can't.".to_string(),
                    ));
                } else {
                    return Ok(true);
                }
            }
            ArchiverMode::Archive => return Ok(true),
            ArchiverMode::List => return Ok(true),
        }
    }
}
