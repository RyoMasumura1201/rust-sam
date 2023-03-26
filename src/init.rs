use std::process::Command;
use std::io;
use tempfile::tempdir;
use dirs::home_dir;


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

    let dest_path = config_dir.join(REPOSITORY_DIR);

    println!("Cloning from https://github.com/aws/aws-sam-cli-app-templates (process may take a moment)");

    Command::new("git")
        .args(["clone", SAM_TEMPLATE_URL, temp_path_str])
        .output()?;

    Ok(temp_path_str.to_string())
}

fn find_template() {
    
}

fn generate_files() {
    println!("generate");
}