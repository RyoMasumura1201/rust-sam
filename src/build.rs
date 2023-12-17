use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
struct Function {
    // Function id, can be Logical ID or any function identifier to define a function in specific IaC
    function_id: String,
    // Function's logical ID (used as Function name below if Property `FunctionName` is not defined)
    name: String,
    // Function name (used in place of logical ID)
    functionname: String,
    // Runtime/language
    runtime: Option<String>,
    // Memory in MBs
    memory: Option<i32>,
    // Function Timeout in seconds
    timeout: Option<i32>,
    // Name of the handler
    handler: Option<String>,
    // Image Uri
    imageuri: Option<String>,
    // Package Type
    packagetype: String,
    // Image Configuration
    imageconfig: Option<String>,
    // Path to the code. This could be a S3 URI or local path or a dictionary of S3 Bucket, Key, Version
    codeuri: Option<String>,
    // Environment variables. This is a dictionary with one key called Variables inside it.
    // This contains the definition of environment variables
    environment: Option<HashMap<String, String>>,
    // Lambda Execution IAM Role ARN. In the future, this can be used by Local Lambda runtime to assume the IAM role
    // to get credentials to run the container with. This gives a much higher fidelity simulation of cloud Lambda.
    rolearn: Option<String>,
    // List of Layers
    // layers: Vec<LayerVersion>,
    // Event
    // events: Option<Vec<Event>>, // Assuming `Event` is a type you have defined
    // Metadata
    metadata: Option<HashMap<String, String>>,
    // InlineCode
    inlinecode: Option<String>,
    // Code Signing config ARN
    codesign_config_arn: Option<String>,
    // Architecture Type
    architectures: Option<Vec<String>>,
    // The function url configuration
    function_url_config: Option<HashMap<String, String>>,
    // FunctionBuildInfo see implementation doc for its details
    // function_build_info: FunctionBuildInfo, // Assuming `FunctionBuildInfo` is a type you have defined
    // The path of the stack relative to the root stack, it is empty for functions in root stack
    stack_path: String,
    // Configuration for runtime management. Includes the fields `UpdateRuntimeOn` and `RuntimeVersionArn` (optional).
    runtime_management_config: Option<HashMap<String, String>>,
    // LoggingConfig for Advanced logging
    logging_config: Option<HashMap<String, String>>,
}

#[derive(Debug)]
struct ResourcesToBuildCollector {
    functions: Vec<Function>,
}

#[derive(Debug)]
struct Stack {
    parent_stack_path: String,
    name: String,
    location: String,
    parameters: Option<HashMap<String, String>>,
    template_dict: HashMap<String, String>,
    metadata: Option<HashMap<String, String>>,
}

#[derive(Debug)]
struct TemplateNotFoundException;

impl fmt::Display for TemplateNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No project template found in the specified directory",)
    }
}

impl Error for TemplateNotFoundException {}

#[derive(Debug)]
struct TemplateFailedParsingException(String);

impl std::fmt::Display for TemplateFailedParsingException {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TemplateFailedParsingException {}

pub fn build() {
    match get_or_default_template_file_name() {
        Ok(path) => {
            println!("Path to template.yaml: {}", path.display());
            run(path);
        }
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

fn run(template_file: PathBuf) {
    collect_build_resources(template_file);
}

fn collect_build_resources(template_file: PathBuf) -> ResourcesToBuildCollector {
    let stacks = get_stacks(template_file);

    let function = Function {
        function_id: "func123".to_string(),
        name: "MyFunction".to_string(),
        functionname: "CustomName".to_string(),
        runtime: Some("nodejs12.x".to_string()),
        memory: Some(128),
        timeout: Some(30),
        handler: Some("index.handler".to_string()),
        imageuri: None,
        packagetype: "Zip".to_string(),
        imageconfig: None,
        codeuri: Some("s3://bucket/key".to_string()),
        environment: None,
        rolearn: Some("arn:aws:iam::123456789012:role/lambda-role".to_string()),
        metadata: None,
        inlinecode: None,
        codesign_config_arn: None,
        architectures: None,
        function_url_config: None,
        stack_path: "root/".to_string(),
        runtime_management_config: None,
        logging_config: None,
    };

    let mut vec = Vec::new();
    vec.push(function);
    ResourcesToBuildCollector { functions: vec }
}

fn get_stacks(template_file: PathBuf) -> Vec<Stack> {
    let template_data = get_template_data(template_file);
    let parent_stack_path = "parent/path".to_string();
    let name = "stack_name".to_string();
    let location = "location/path".to_string();
    let parameters = Some(HashMap::new());
    let template_dict = HashMap::new();
    let metadata = Some(HashMap::new());
    let stack = Stack {
        parent_stack_path,
        name,
        location,
        parameters,
        template_dict,
        metadata,
    };
    let mut vec = Vec::new();
    vec.push(stack);
    vec
}

fn get_template_data(template_file: PathBuf) -> Result<serde_yaml::Value, Box<dyn Error>> {
    if !template_file.exists() {
        return Err(Box::new(TemplateNotFoundException));
    }

    let mut file = File::open(&template_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    serde_yaml::from_str::<serde_yaml::Value>(&contents).map_err(|ex| {
        Box::new(TemplateFailedParsingException(format!(
            "Failed to parse template: {}",
            ex
        ))) as Box<dyn Error>
    })
}
