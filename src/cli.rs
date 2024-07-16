use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(
    version, author, about,
    arg_required_else_help = true,
)]
pub struct CliOpts{
    #[clap(
        short="m",
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
        short="m",
        long="mode",
        default_value_t=RunMode::Wizard,
        value_name="MODE",
        required=false,
        ignore_case = true, 
        value_enum, 
        help = "Mode of operation."
    )]
    pub source:PathBuf,
    pub dest:PathBuf,
    pub overwrite:bool,
    pub recursive:bool,
    pub verbose:bool,
    pub args:Vec<Pathbuf>
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Copy)]
pub enum RunMode {
    Auto,
    Archive,
    Extract,
    Wizard,
}

