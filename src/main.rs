use clap::Parser;
mod cli;
use cli::{Commands, Cli};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { name }) => {
            println!("'myapp init' was used");
            match name {
                Some(name) => println!("name is {}", name),
                None => println!("name is not given"),
            }
        }
        None => {}
    }
}