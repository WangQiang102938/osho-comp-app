use crate::format::FileFormat;
use std::collections::HashMap;

pub trait Archiver {
    fn exec_archive(&self);
    fn exec_extract(&self);
    fn archive_support_check(format: FileFormat, mode: ArchiverMode) -> bool;
    fn avaliable_options(mode: ArchiverMode) -> HashMap<String, String>;
}

pub enum ArchiverMode {
    Archive,
    Extract,
}

pub struct ArchiveOptions {
    source_paths: Vec<PathBuf>,
    target_paths: Vec<PathBuf>,
    options: HashMap<String, String>,
}


