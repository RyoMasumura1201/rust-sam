use std::env;
use std::path::PathBuf;

pub fn build() {
    match get_or_default_template_file_name() {
        Ok(path) => println!("Path to template.yaml: {}", path.display()),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_or_default_template_file_name() -> Result<PathBuf, String> {
    match env::current_dir() {
        Ok(mut path) => {
            path.push("template.yaml");
            if path.exists() {
                Ok(path)
            } else {
                Err("template.yaml not found in the current directory.".to_string())
            }
        }
        Err(e) => Err(format!("Failed to get current directory: {}", e)),
    }
}
