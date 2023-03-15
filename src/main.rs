use clap::Parser;
mod cli;
use cli::{Commands, Cli};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { }) => {
            println!("'myapp init' was used")
        }
        None => {}
    }
}