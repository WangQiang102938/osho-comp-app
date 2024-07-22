use std::collections::HashMap;

use sevenz::Archiver7z;

use crate::{shared_utils::AppResult, task::ArchiveJob};

pub mod sevenz;

pub fn get_archivers()->Vec<Box<dyn Archiver>>{
    let mut archiver_list:Vec<Box<dyn Archiver>>=Vec::new();
    archiver_list.push(Box::new(Archiver7z{}));
    return archiver_list;
}

pub trait Archiver: std::fmt::Debug {

    // job self check and archiver check garenteed correctness.
    fn exec(&self, job: &ArchiveJob) -> AppResult<bool>;
    fn job_check(&self, job: &ArchiveJob) -> bool;
    fn archive_support_check(&self, path: String, mode: ArchiverMode) -> bool;
    fn avaliable_options(&self, mode: ArchiverMode) -> HashMap<String, String>;
}

#[derive(Debug)]
pub struct DummyArchiver {}

#[allow(unused,unused_variables)]
impl Archiver for DummyArchiver {
    fn archive_support_check(&self, format: String, mode: ArchiverMode) -> bool {
        return true;
    }

    fn avaliable_options(&self, mode: ArchiverMode) -> HashMap<String, String> {
        return HashMap::new();
    }

    fn exec(&self, job: &ArchiveJob) -> AppResult<bool> {
        todo!()
    }

    fn job_check(&self, job: &ArchiveJob) -> bool {
        todo!()
    }
}

#[derive(PartialEq, Debug)]
pub enum ArchiverMode {
    Archive,
    Extract,
    List,
    Unknown,
}
