use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: CliCmds,
    #[arg(short, long, help = "prints the compilation log fully")]
    pub verbose: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CliCmds {
    #[command(about = "creates a new project or inits a directory if path isn't provided")]
    New {
        path: Option<std::path::PathBuf>,
    },
    #[command(about="build then run")]
    Run,
    #[command(about="builds the project and adds dependencies to config")]
    Build,
    #[command(about="cleans the target directory")]
    Clean,
    #[command(about = "prints the full config in pretty format")]
    Info,
    #[command(about = "adds dependencies to the config file")]
    Add{
        #[arg(short, long, help ="enable global pick")]
        glob:bool

    },
}