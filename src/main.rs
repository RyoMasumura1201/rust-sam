use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { }) => {
            println!("'myapp init' was used")
        }
        None => {}
    }
}