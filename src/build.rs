use std::collections::HashMap;
use std::env;
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

fn run(template_file: PathBuf) {}

fn collect_build_resources() -> ResourcesToBuildCollector {}

fn get_stacks(template_file: PathBuf) {}
