use clap::Parser;
mod cli;
use cli::{Cli, Commands};
pub mod config;
pub mod cookiecutter;
mod init;
use crate::config as global_config;

fn main() {
    global_config::create_config_directory_if_not_exists()
        .expect("configディレクトリ作成に失敗しました");
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { name }) => {
            println!("'myapp init' was used");
            match name {
                Some(name) => {
                    println!("name is {}", name);
                    match init::init(name) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("init failed {:?}", e);
                        }
                    }
                }
                None => {
                    println!("name is not given");
                    match init::init("sam-app") {
                        Ok(_) => {}
                        Err(e) => {
                            println!("init failed {:?}", e);
                        }
                    }
                }
            }
        }
        None => {}
    }
}
