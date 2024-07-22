use std::collections::HashMap;

mod sevenz;

pub trait Archiver: std::fmt::Debug {
    fn exec_archive(&self);
    fn exec_extract(&self);
    fn archive_support_check(&self, format: String, mode: ArchiverMode) -> bool;
    fn avaliable_options(&self, mode: ArchiverMode) -> HashMap<String, String>;
}

#[derive(Debug)]
pub struct DummyArchiver {}

impl Archiver for DummyArchiver {
    fn exec_archive(&self) {}

    fn exec_extract(&self) {}

    fn archive_support_check(&self, format: String, mode: ArchiverMode) -> bool {
        return true;
    }

    fn avaliable_options(&self, mode: ArchiverMode) -> HashMap<String, String> {
        return HashMap::new();
    }
}

#[derive(PartialEq, Debug)]
pub enum ArchiverMode {
    Archive,
    Extract,
    Unknown,
}
