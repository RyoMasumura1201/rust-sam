use glob;
use serde_json::{json, Value};
use std::env;
use std::fmt;
use std::fs::{create_dir, read_dir, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tera;
use walkdir::WalkDir;

#[derive(Debug)]
struct NonTemplatedInputDirError;

impl fmt::Display for NonTemplatedInputDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No project template found in the specified directory")
    }
}

#[derive(Debug)]
struct OutputDirExistsError;

impl fmt::Display for OutputDirExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "output directory already exists")
    }
}

impl std::error::Error for OutputDirExistsError {}

pub fn cookiecutter(
    template: PathBuf,
    extra_context: Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let context_file = template.join("cookiecutter.json");
    let context_file = context_file.as_path();

    let mut context = generate_context(context_file, extra_context)?;

    if let Some(Value::Object(ref mut cookiecutter_map)) = context.get_mut("cookiecutter") {
        cookiecutter_map.insert("_template".to_string(), json!(template.to_str()));
        cookiecutter_map.insert(
            "_output_dir".to_string(),
            json!(env::current_dir()
                .expect("failed to get current dir")
                .to_str()),
        );
    }

    println!("{:?}", context);

    generate_files(template, context, env::current_dir()?)?;
    Ok(())
}

fn generate_context(
    context_file: &Path,
    extra_context: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(context_file)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut obj: Value = serde_json::from_str(&contents)?;

    apply_overwrites_to_context(&mut obj, extra_context);

    println!("{:#?}", obj);
    let context: Value = json!({ "cookiecutter": obj });

    Ok(context)
}

fn apply_overwrites_to_context(context: &mut Value, overwrite_context: Value) {
    if let (Value::Object(ref mut context_map), Value::Object(ref overwrite_map)) =
        (context, overwrite_context)
    {
        for (key, overwrite_value) in overwrite_map {
            context_map.insert(key.clone(), overwrite_value.clone());
        }
    }
}

fn generate_files(
    repo_dir: PathBuf,
    context: Value,
    output_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let template_dir = find_template(&repo_dir)?;
    let unrendered_dir = template_dir
        .as_path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let project_dir = render_and_create_dir(unrendered_dir, &context, &output_dir)?;

    let dont_render_list = &context["cookiecutter"]["_copy_without_render"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<Vec<&str>>();

    for entry in WalkDir::new(&template_dir).min_depth(1) {
        let mut copy_dirs: Vec<&str> = vec![];
        let mut render_dirs: Vec<&str> = vec![];

        let entry = entry?;

        if entry.file_type().is_dir() {
            if is_copy_only_path(entry.file_name().to_str().unwrap(), dont_render_list) {
                copy_dirs.push(entry.path().strip_prefix(&template_dir)?.to_str().unwrap());
            } else {
                render_dirs.push(entry.path().strip_prefix(&template_dir)?.to_str().unwrap());
            }
        }

        for render_dir in render_dirs {
            match render_and_create_dir(render_dir, &context, &project_dir) {
                Ok(_) => (),
                Err(e) => println!("Unable to create directory {:?}", e),
            }
        }
    }

    for entry in WalkDir::new(&template_dir).min_depth(1) {
        let entry = entry?;

        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_str().unwrap();
            if is_copy_only_path(file_name, dont_render_list) {
                let tera_context = tera::Context::from_value(context.clone())?;

                let mut tera = tera::Tera::default();
                let outfile_rendered = tera.render_str(file_name, &tera_context)?;
                let outfile = project_dir
                    .join(entry.path().strip_prefix(&template_dir)?)
                    .with_file_name(outfile_rendered);
            } else {
            }
        }
    }

    Ok(())
}

impl std::error::Error for NonTemplatedInputDirError {}

fn find_template(repo_dir: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut project_template: Option<PathBuf> = None;
    for entry in read_dir(repo_dir)? {
        let entry = entry?;
        if let Some(entry_name) = entry.file_name().to_str() {
            if entry_name.contains("cookiecutter")
                && entry_name.contains("{{")
                && entry_name.contains("}}")
            {
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
        }
        None => Err(Box::new(NonTemplatedInputDirError)),
    }
}

fn render_and_create_dir(
    dirname: &str,
    context: &Value,
    output_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let tera_context = tera::Context::from_value(context.clone())?;

    let mut tera = tera::Tera::default();
    let rendered_dirname = tera.render_str(dirname, &tera_context)?;

    println!("{:?}", rendered_dirname);

    let dir_to_create = output_dir.join(rendered_dirname);

    if dir_to_create.exists() {
        return Err(Box::new(OutputDirExistsError));
    }

    create_dir(&dir_to_create)?;

    Ok(dir_to_create)
}

fn is_copy_only_path(path: &str, dont_render_list: &Vec<&str>) -> bool {
    for dont_render in dont_render_list {
        if glob::Pattern::new(dont_render).unwrap().matches(path) {
            return true;
        }
    }

    return false;
}
