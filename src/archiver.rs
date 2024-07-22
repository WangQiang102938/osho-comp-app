
use std::collections::HashMap;



pub trait Archiver {
    fn exec_archive(&self);
    fn exec_extract(&self);
    fn archive_support_check(&self,format: String, mode: ArchiverMode) -> bool;
    fn avaliable_options(&self,mode: ArchiverMode) -> HashMap<String, String>;
}

pub struct DummyArchiver{

}

impl Archiver for DummyArchiver{
    fn exec_archive(&self) {
        todo!()
    }

    fn exec_extract(&self) {
        todo!()
    }

    fn archive_support_check(&self,format: String, mode: ArchiverMode) -> bool {
        todo!()
    }

    fn avaliable_options(&self,mode: ArchiverMode) -> HashMap<String, String> {
        todo!()
    }
}

pub enum ArchiverMode {
    Archive,
    Extract,
    Unknown,
}




