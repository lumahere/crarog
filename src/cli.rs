use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cli{
    #[command(subcommand)]
    pub cmd:CliCmds,
    #[arg(short, long, help="prints the compilation log fully")]
    pub verbose:bool
}

#[derive(Subcommand, Debug, Clone)]
pub enum CliCmds{
    #[command(about="creates a new project")]
    New{
        path:Option<std::path::PathBuf>
    },
    Build,
    #[command(about="prints the full config in pretty format")]
    Info
}
