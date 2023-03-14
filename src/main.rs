use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init { name: Option<String> },
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { name }) => {
            println!("'myapp init' was used, name is: {name:?}")
        }
        None => {}
    }
}

// use clap::{Parser, Subcommand};

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// #[command(propagate_version = true)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Adds files to myapp
//     Add { name: Option<String> },
// }

// fn main() {
//     let cli = Cli::parse();

//     // You can check for the existence of subcommands, and if found use their
//     // matches just as you would the top level cmd
//     match &cli.command {
//         Commands::Add { name } => {
//             println!("'myapp add' was used, name is: {name:?}")
//         }
//     }
// }