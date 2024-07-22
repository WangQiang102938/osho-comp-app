
use std::collections::HashMap;



pub trait Archiver {
    fn exec_archive(&self);
    fn exec_extract(&self);
    fn archive_support_check(&self,format: String, mode: ArchiverMode) -> bool;
    fn avaliable_options(&self,mode: ArchiverMode) -> HashMap<String, String>;
}

pub enum ArchiverMode {
    Archive,
    Extract,
}




