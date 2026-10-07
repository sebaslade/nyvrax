mod cli;
mod commands;

use std::error::Error;

use clap::Parser;

use cli::{Cli, Commands};

fn main() {
    let exit_code = match run() {
        Ok(code) => code,

        Err(error) => {
            eprintln!("nyvrax: {error}");

            2
        }
    };

    std::process::exit(exit_code);
}

fn run() -> Result<i32, Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            changed,
            base,
            format,
        } => commands::scan::execute(changed, base, format),

        Commands::Version => Ok(commands::version::execute()),
    }
}
