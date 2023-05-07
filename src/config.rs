use dirs::home_dir;
use serde::Deserialize;
use std::fs::{self, File};
use std::io::{self, Read};

pub const SAM_TEMPLATE_URL: &str = "https://github.com/aws/aws-sam-cli-app-templates.git";

pub const REPOSITORY_DIR: &str = "aws-sam-cli-app-templates";

pub const CONFIG_DIR: &str = ".aws-rsam";

#[derive(Debug, Deserialize)]
struct RuntimeConfig {
    app_template_repo_commit: String,
}

pub fn get_app_template_repo_commit() -> String {
    let mut file =
        File::open("src/runtime_config.json").expect("Failed to open runtime_config.json");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read runtime_config.json");

    let config: RuntimeConfig =
        serde_json::from_str(&contents).expect("Failed to deserialize runtime_config.json");

    config.app_template_repo_commit
}

// [TODO] metadata.json作成
pub fn create_config_directory_if_not_exists() -> io::Result<()> {
    let home_dir = home_dir().expect("failed to read home directory");
    let config_dir = home_dir.join(CONFIG_DIR);
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }
    Ok(())
}
