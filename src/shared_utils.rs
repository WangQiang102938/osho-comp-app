use std::path::PathBuf;

pub type AppResult<T>=std::result::Result<T,AppError>;

#[derive(Debug)]
pub enum AppError{
    NoArgumentsGiven,
    FileNotFound(PathBuf),
    FileExists(PathBuf),
    IO(std::io::Error),
    Archiver(String),
    UnsupportedFormat(String),
    UnknownFormat(String),
    Unknown(String),
    Fatal(Box<dyn std::error::Error>)    
}

pub fn get_supported_format(is_extract:bool)->Vec<String>{
    // TODO: let archiver report supported format
    let tmp_list=vec![".zip", ".tar", ".tar.gz", ".tgz", ".tar.bz2", ".tbz2", ".rar", ".jar", ".war", ".ear", "7z", ];
    return tmp_list.iter().map(|&s|s.to_string()).collect();
}