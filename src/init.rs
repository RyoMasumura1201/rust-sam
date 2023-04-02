use std::{process::Command, path::PathBuf, path::Path, io};
use fs_extra::error as fs_extra_error;
use tempfile::tempdir;
use dirs::home_dir;
use fs_extra::dir::{copy, CopyOptions};

use crate::config::{SAM_TEMPLATE_URL, REPOSITORY_DIR, CONFIG_DIR, get_app_template_repo_commit};

pub fn init() {
    match clone_templates_repo() {
        Ok(repo_dir) => {
            println!("{:?}", repo_dir);
            generate_files();
        }
        Err(e)=> {
            eprintln!("git clone failed with error:{:?}", e);
        }
    }
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

    match clone_command.output() {
        Ok(_)=>(),
        Err(e)=> return Err(CloneError::IoError(e))
    }   
    
    let mut checkout_command = Command::new("git");
    checkout_command.args(["checkout", commit]);
    checkout_command.current_dir(temp_path_str);

    match checkout_command.output() {
        Ok(_)=>(),
        Err(e)=> return Err(CloneError::IoError(e))
    }   

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
        Ok(_) => false,
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound=> {
                    eprintln!("Cache directory does not yet exist, creating one.");
                },
                _=> {
                    eprintln!("rev-parse  {:?}", e);
                }
            }
            true
        }
    }
}

fn generate_files() {
    println!("generate");
}