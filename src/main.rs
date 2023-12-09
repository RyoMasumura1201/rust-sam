use clap::Parser;
mod cli;
use cli::{Cli, Commands};
mod build;
pub mod config;
pub mod cookiecutter;
mod init;
use crate::config as global_config;
use dialoguer::{theme::ColorfulTheme, Input};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    global_config::create_config_directory_if_not_exists()
        .expect("failed to create config directory");
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { name }) => match name {
            Some(name) => {
                init::init(name)?;
            }
            None => {
                let project_name: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Project name")
                    .default("sam-app".to_string())
                    .interact_text()
                    .unwrap();

                init::init(project_name.as_str())?;
            }
        },
        Some(Commands::Build) => build::build(),
        None => {}
    }

    Ok(())
}
