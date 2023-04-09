use std::{path::PathBuf, path::Path};
use serde_json::{json, Value};
use std::fs::File;
use std::io::Read;


pub fn cookiecutter(template: PathBuf, extra_context: Value){

    let context_file = template.join("cookiecutter.json");
    let context_file = context_file.as_path();

    generate_context(context_file, extra_context)
}

fn generate_context(context_file: &Path, extra_context: Value){
    let mut file = File::open(context_file)
        .expect("Failed to open context file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read context file");

    let mut obj: Value = serde_json::from_str(&contents).expect("faile to create context object");

    apply_overwrites_to_context(&mut obj, extra_context);

    println!("{:#?}", obj);
}

fn apply_overwrites_to_context(context: &mut Value, overwrite_context: Value){
    if let (Value::Object(ref mut context_map), Value::Object(ref overwrite_map)) = (context, overwrite_context) {
        for (key, overwrite_value) in overwrite_map {
            context_map.insert(key.clone(), overwrite_value.clone());
        }
    }
}