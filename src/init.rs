use std::process::Command;
use std::path::PathBuf;
use std::io;

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

    const REPOSITORY_DIR: &str = ".aws-sam-cli-app-templates";

    let clone_to_dir = PathBuf::from(".");

    let repo_dir = clone_to_dir.join(REPOSITORY_DIR);
    let repo_dir_str = repo_dir.as_path().to_str().expect("Failed to convert PathBuf to str");

    println!("Cloning from https://github.com/aws/aws-sam-cli-app-templates (process may take a moment)");

    Command::new("git")
        .args(["clone", SAM_TEMPLATE_URL, repo_dir_str])
        .output()?;

    Ok(repo_dir_str.to_string())
}

fn generate_files() {
    println!("generate");
}