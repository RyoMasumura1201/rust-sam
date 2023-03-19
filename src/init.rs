use std::process::Command;

pub fn clone() {
    const SAM_TEMPLATE_URL: &str = "https://github.com/aws/aws-sam-cli-app-templates.git";

    const REPOSITORY_URL: &str = "aws-sam-cli-app-templates";

    let clone_to_dir = ".".to_string() + REPOSITORY_URL;

    println!("Cloning from https://github.com/aws/aws-sam-cli-app-templates (process may take a moment)");

    Command::new("git")
        .args(["clone", SAM_TEMPLATE_URL, &clone_to_dir])
        .output()
        .expect("failed to clone repository");
}