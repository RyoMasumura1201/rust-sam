use std::path::{PathBuf, Path};
use serde_json::{json, Value};
use std::fs::{File, read_dir};
use std::io::{self,Read};
use std::fmt;
use std::env;


pub fn cookiecutter(template: PathBuf, extra_context: Value)-> Result<(), Box<dyn std::error::Error>>{

    let context_file = template.join("cookiecutter.json");
    let context_file = context_file.as_path();

    let mut context = generate_context(context_file, extra_context)?;

    if let Some(Value::Object(ref mut cookiecutter_map)) = context.get_mut("cookiecutter"){
        cookiecutter_map.insert("_template".to_string(), json!(template.to_str()));
        cookiecutter_map.insert("_output_dir".to_string(), json!(env::current_dir().expect("failed to get current dir").to_str()));
    }

    println!("{:?}", context);

    generate_files(template, context, env::current_dir()?);
    Ok(())
}

fn generate_context(context_file: &Path, extra_context: Value)-> Result<Value, Box<dyn std::error::Error>>{
    let mut file = File::open(context_file)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut obj: Value = serde_json::from_str(&contents)?;

    apply_overwrites_to_context(&mut obj, extra_context);

    println!("{:#?}", obj);
    let context: Value = json!({
        "cookiecutter": obj
    });

    Ok(context)
}

fn apply_overwrites_to_context(context: &mut Value, overwrite_context: Value){
    if let (Value::Object(ref mut context_map), Value::Object(ref overwrite_map)) = (context, overwrite_context) {
        for (key, overwrite_value) in overwrite_map {
            context_map.insert(key.clone(), overwrite_value.clone());
        }
    }
}

fn generate_files(repo_dir: PathBuf, context: Value, output_dir: PathBuf)-> Result<(), Box<dyn std::error::Error>>{
    let template_dir = find_template(&repo_dir)?;
    println!("{:?}", template_dir);
    Ok(())
}

#[derive(Debug)]
struct NonTemplatedInputDirError;

impl fmt::Display for NonTemplatedInputDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No project template found in the specified directory")
    }
}

impl std::error::Error for NonTemplatedInputDirError{}

fn find_template(repo_dir: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>>{
    let mut project_template: Option<PathBuf> = None;
    for entry in read_dir(repo_dir)? {
        let entry = entry?;
        if let Some(entry_name) =entry.file_name().to_str(){
            if entry_name.contains("cookiecutter") && entry_name.contains("{{") && entry_name.contains("}}") {
                project_template = Some(entry.path());
                break;
            }
        }   
    }

    match project_template {
        Some(template) => {
            let project_template = repo_dir.join(template);
            println!("{:?}", project_template);
            Ok(project_template)
        },
        None=>Err(Box::new(NonTemplatedInputDirError)),
    }
}