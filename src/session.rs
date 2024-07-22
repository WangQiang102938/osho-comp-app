use crate::archiver::{Archiver, ArchiverMode};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct AppSession {
    jobs: Vec<ArchiveJob>,
    options: HashMap<String, String>,
}

pub struct ArchiveJob {
    source_paths: Vec<PathBuf>,
    target_path: PathBuf,
    archiver: Box<dyn Archiver>,
    mode: ArchiverMode,
    overwrite:bool,
    with_creation:bool,
    options: HashMap<String, String>,
}
