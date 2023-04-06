use std::{path::PathBuf, path::Path};

#[derive(Debug)]
pub struct Architectures {
    pub value: Vec<String>
}

#[derive(Debug)]
pub struct ExtraContext {
    pub project_name: String,
    pub runtime: String,
    pub architectures: Architectures
}

pub fn cookiecutter(template: PathBuf, extra_context: ExtraContext){

    let context_file = template.join("cookiecutter.json");
}