use clap::Parser;

mod commands;
mod parsing;
mod cli;

fn main() {
    let env = self::cli::Cli::parse();
    
    match env.cmd{
        cli::CliCmds::New { path } => {self::commands::new(path);}
        cli::CliCmds::Build => {self::commands::build(env.verbose);},
        cli::CliCmds::Info => {self::commands::info();}
    }
}
