use std::{process::Command, path::PathBuf};
use std::io;
use fs_extra::error;
use tempfile::tempdir;
use dirs::home_dir;
use fs_extra::dir::{copy, CopyOptions};


const CONFIG_DIR: &str = ".aws-rsam";

pub fn init() {
    match clone() {
        Ok(repo_dir_str) => {
            println!("{}", repo_dir_str);
            generate_files();
        }
        Err(e)=> {
            eprintln!("git clone failed with error:{}", e);
        }
    }
}

fn clone() -> Result<String, io::Error> {
    const SAM_TEMPLATE_URL: &str = "https://github.com/aws/aws-sam-cli-app-templates.git";

    const REPOSITORY_DIR: &str = "aws-sam-cli-app-templates";

    let home_dir = home_dir().expect("failed to read home directory");

    let config_dir = home_dir.join(CONFIG_DIR);

    println!("{:?}", config_dir);

    let temp_dir = tempdir()?;

    let temp_path = temp_dir.path().join(REPOSITORY_DIR);

    println!("{:?}",temp_path);

    let temp_path_str = temp_path.as_path().to_str().expect("Failed to convert PathBuf to str");

    println!("Cloning from https://github.com/aws/aws-sam-cli-app-templates (process may take a moment)");

    Command::new("git")
        .args(["clone", SAM_TEMPLATE_URL, temp_path_str])
        .output()?;


    Ok(temp_path_str.to_string())
}

fn persist_local_repo(temp_path: &str, dest_dir: PathBuf, dest_name: &str) -> Result<PathBuf, error::Error>{
    let dest_path = dest_dir.join(dest_name);
    let options = CopyOptions::new();

    copy(temp_path, &dest_path, &options)?;

    Ok(dest_path)
}

fn generate_files() {
    println!("generate");
}