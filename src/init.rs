use std::process::Command;

pub fn clone() {
    const SAM_TEMPLATE_URL: &str = "https://github.com/aws/aws-sam-cli-app-templates.git";

    println!("Cloning from https://github.com/aws/aws-sam-cli-app-templates (process may take a moment)");

    Command::new("git")
        .args(["clone", SAM_TEMPLATE_URL])
        .output()
        .expect("failed to clone repository");
}