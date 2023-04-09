use std::{path::PathBuf, path::Path};
use serde_json::{json, Value};
use std::fs::File;
use std::io::Read;
use std::env;


pub fn cookiecutter(template: PathBuf, extra_context: Value){

    let context_file = template.join("cookiecutter.json");
    let context_file = context_file.as_path();

    let mut context = generate_context(context_file, extra_context);

    if let Some(Value::Object(ref mut cookiecutter_map)) = context.get_mut("cookiecutter"){
        cookiecutter_map.insert("_template".to_string(), json!(template.to_str()));
        cookiecutter_map.insert("_output_dir".to_string(), json!(env::current_dir().expect("failed to get current dir").to_str()));
    }

    println!("{:?}", context);
}

fn generate_context(context_file: &Path, extra_context: Value)-> Value{
    let mut file = File::open(context_file)
        .expect("Failed to open context file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read context file");

    let mut obj: Value = serde_json::from_str(&contents).expect("faile to create context object");

    apply_overwrites_to_context(&mut obj, extra_context);

    println!("{:#?}", obj);
    let context: Value = json!({
        "cookiecutter": obj
    });

    context
}

fn apply_overwrites_to_context(context: &mut Value, overwrite_context: Value){
    if let (Value::Object(ref mut context_map), Value::Object(ref overwrite_map)) = (context, overwrite_context) {
        for (key, overwrite_value) in overwrite_map {
            context_map.insert(key.clone(), overwrite_value.clone());
        }
    }
}