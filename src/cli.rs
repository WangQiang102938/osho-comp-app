use std::path::PathBuf;
use clap::{Parser, ValueEnum};
use crate::shared_utils::{AppResult,AppError,get_supported_format};


#[derive(Parser, Debug)]
#[clap(
    version, author, about,
    arg_required_else_help = true,
)]
pub struct CliOpts{
    #[clap(
        short='m',
        long="mode",
        default_value_t=RunMode::Wizard,
        value_name="MODE",
        required=false,
        ignore_case = true, 
        value_enum, 
        help = "Mode of operation."
    )]
    pub mode:RunMode,
    #[clap(
        short='s',
        long="src",
        value_name="SOURCE",
        required=false,
        help = "Path(s) of source. default is current directory"
    )]
    pub source:Option<PathBuf>,
    #[clap(
        short='d',
        long="dest",
        value_name="DEST",
        required=false,
        help = "Path(s) of destination. default is directory/file name"
    )]
    pub dest:Option<PathBuf>,
    #[clap(long, help = "Overwrite existing files.")]
    pub overwrite:bool,
    #[clap(long, help = "Disable recursive directory (archive only)")]
    pub no_recursive:bool,
    #[clap(long, help = "Enable verbose mode")]
    pub verbose:bool,
    #[clap(value_name = "ARGUMENTS", help = "List of files or directories to be processed.")]
    pub args:Vec<PathBuf>,
}

impl CliOpts{
    pub fn check_runmode(&mut self)->AppResult<RunMode>{
        if(self.args.len()==0){
            return Err(AppError::NoArgumentsGiven)
        }
        if(self.mode==RunMode::Wizard){
            return Ok(RunMode::Wizard)
        }
        if(self.mode==RunMode::Auto){
            let result= self.args.iter().all(|arg|{
                let name=arg.to_str().unwrap().to_lowercase();
                let exts=get_supported_format(true);
                for ext in exts.iter(){
                    if name.ends_with(ext){
                        return true;
                    }
                }
                return false;
            });
            if result{
                self.mode=RunMode::Extract;
            }else{
                self.mode=RunMode::Archive;
            }
        }
        return Ok(self.mode);
    }

    
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum RunMode {
    Auto,
    Archive,
    Extract,
    Wizard,
}

