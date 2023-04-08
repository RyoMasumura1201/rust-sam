use std::{path::PathBuf, path::Path};
use serde::Deserialize;
use serde_json::{json, Value};


pub fn cookiecutter(template: PathBuf, extra_context: Value){

    let context_file = template.join("cookiecutter.json");
    let context_file = context_file.as_path();

    generate_context(context_file, extra_context)
}

fn generate_context(context_file: &Path, extra_context: Value){

}