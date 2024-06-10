use clap::Parser;
mod cli;
mod commands;
mod parsing;
mod buildlogic;
mod macros; 


fn main() {
    let env = self::cli::Cli::parse();

    match env.cmd {
        cli::CliCmds::New { path } => {
            self::commands::new(path);
        }
        cli::CliCmds::Build => {
            self::buildlogic::build(env.verbose);
        }
        cli::CliCmds::Info => {
            self::commands::info();
        }
        cli::CliCmds::Clean => {self::commands::clean()},
        cli::CliCmds::Run => {self::commands::run(env.verbose)},
        cli::CliCmds::Add {glob}=> {self::commands::add(glob)}
    }
}
