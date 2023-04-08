use std::{process::Command, path::PathBuf, path::Path, io};
use fs_extra::error as fs_extra_error;
use tempfile::tempdir;
use dirs::home_dir;
use fs_extra::dir::{copy, CopyOptions};
use serde_json::{json, Value};

use crate::config::{SAM_TEMPLATE_URL, REPOSITORY_DIR, CONFIG_DIR, get_app_template_repo_commit};

use crate::cookiecutter::{cookiecutter};

pub fn init(name: &str) {
    match clone_templates_repo() {
        Ok(_) => {
            println!("cloned");
        }
        Err(e)=> {
            eprintln!("git clone failed with error:{:?}", e);
        }
    }

    let location = home_dir().expect("failed to read home directory").join(CONFIG_DIR).join(REPOSITORY_DIR).join("python3.9/hello");
    println!("{:?}", location);

    let extra_context = json!({
        "project_name": name.to_string(),
        "runtime": "python3.9".to_string(),
        "architectures":  vec!["x86_64".to_string()]
    });

    generate_project(location, extra_context);
}

#[derive(Debug)]
enum CloneError {
    IoError(io::Error),
    FsError(fs_extra_error::Error)
}

fn clone_templates_repo()-> Result<(), CloneError> {
    let home_dir = home_dir().expect("failed to read home directory");

    let config_dir = home_dir.join(CONFIG_DIR);

    println!("{:?}", config_dir);

    if !check_upsert_templates(&config_dir, REPOSITORY_DIR) {
        print!("check");
        return Ok(());
    }

    let commit = get_app_template_repo_commit();

    clone(&config_dir, &commit)?;

    Ok(())
}

fn clone(config_dir: &PathBuf, commit: &str) -> Result<PathBuf, CloneError> {
    let temp_dir = tempdir().map_err(CloneError::IoError)?;

    let temp_path = temp_dir.path().join(REPOSITORY_DIR);

    println!("{:?}",temp_path);

    let temp_path_str = temp_path.as_path().to_str().expect("Failed to convert PathBuf to str");

    println!("Cloning from {} (process may take a moment)", SAM_TEMPLATE_URL);

    let mut clone_command = Command::new("git");
    clone_command.args(["clone", SAM_TEMPLATE_URL]);
    clone_command.current_dir(temp_dir.path());

    clone_command.output().map_err(CloneError::IoError)?;

    checkout_commit(temp_path_str, commit).map_err(CloneError::IoError)?;

    match persist_local_repo(temp_path_str, config_dir, REPOSITORY_DIR) {
        Ok(path) => Ok(path),
        Err(e) => Err(CloneError::FsError(e)),
    }
}

fn persist_local_repo(temp_path: &str, dest_dir: &PathBuf, dest_name: &str) -> Result<PathBuf, fs_extra_error::Error>{
    let dest_path = dest_dir.join(dest_name);
    let options = CopyOptions::new();

    copy(temp_path, dest_dir, &options)?;

    Ok(dest_path)
}

fn check_upsert_templates(shared_dir: &Path, cloned_folder_name: &str)->bool{
    let cache_dir = shared_dir.join(cloned_folder_name);
    let mut command = Command::new("git");
    command.args(["rev-parse", "--verify", "HEAD"]);
    command.current_dir(cache_dir);

    match command.output() {
        Ok(output) => {
            if output.status.success(){
                println!("Existing hash: {:?}", String::from_utf8_lossy(&output.stdout).trim());
                let existing_hash = String::from_utf8_lossy(&output.stdout).trim().to_owned();
                existing_hash != get_app_template_repo_commit()
            } else {
                eprintln!("Unable to check existing cache hash\n{:?}", output.stderr);
                true
            }
        },
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound=> {
                    eprintln!("Cache directory does not yet exist, creating one.");
                },
                _=> {
                    eprintln!("Unable to check existing cache hash\n{:?}", e);
                }
            }
            true
        }
    }
}


fn checkout_commit (repo_dir: &str, commit: &str)-> Result<(), io::Error> {
    let mut checkout_command = Command::new("git");
    checkout_command.args(["checkout", commit]);
    checkout_command.current_dir(repo_dir);

    checkout_command.output()?;
    Ok(())
}

fn generate_project(location: PathBuf, extra_context: Value) {
    println!("generate");
    cookiecutter(location, extra_context);
}