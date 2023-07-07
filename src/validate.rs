use clap::Args;
use jsonschema::JSONSchema;
use module::module::Module;
use module::TYPE_KEY;
use serde_json::{json, Value};
use std::io::BufReader;
use std::{error::Error, fs::File, path::PathBuf};
use tracing::{debug, error, info, instrument};

const MODULE_SCHEMA: &str = "https://specification.powerd6.org/schemas/module.json";
const CONTENT_SCHEMA: &str = "https://specification.powerd6.org/schemas/content.json";

/// Validates a module based on the global schemas and the local types.
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct ValidateArguments {
    /// The path to the module to be rendered.
    #[arg(required = true)]
    source: PathBuf,
}

/// Executes the [Validate](crate::Commands::Validate) command.
pub fn run(ValidateArguments { source }: ValidateArguments) -> Result<(), Box<dyn Error>> {
    info!("Starting to validate the module");
    let file = File::open(source)?;
    let reader = BufReader::new(file);
    let module: Module = serde_json::from_reader(reader)?;
    let module_value = serde_json::to_value(&module)?;
    debug!("Loaded module from file correctly: {:#?}", module);
    validate_module_schema(&module_value)?;
    validate_contents(module)?;
    info!("Module validated!");
    Ok(())
}

#[instrument(skip(module))]
fn validate_contents(module: Module) -> Result<(), Box<dyn Error>> {
    info!("Validating module contents");
    if let Some(contents) = module.contents {
        for (content_id, content) in contents {
            let content_type = content
                .get(TYPE_KEY)
                .expect("The content should have a defined type.")
                .as_str()
                .expect("The content type should be a string.");
            let content_value =
                serde_json::to_value(&content).expect("The content should be serializable.");
            if let Some(ref module_types) = module.types {
                if let Some(module_type) = module_types.get(content_type) {
                    if let Some(type_schema) = &module_type.schema {
                        validate_content(content_id, &content_value, type_schema)?;
                    }
                }
            }
        }
    }
    Ok(())
}

#[instrument(skip(content, type_schema))]
fn validate_content(
    content_id: String,
    content: &Value,
    type_schema: &Value,
) -> Result<(), Box<dyn Error>> {
    debug!("Validating content {}", content_id);
    let content_schema = json!({
      "anyOf": [
        {
          "$ref": CONTENT_SCHEMA
        },
        type_schema
      ],
      "unevaluatedProperties": false
    });
    let validator =
        JSONSchema::compile(&content_schema).expect("The type should have a valid schema.");
    let result = validator.validate(content);
    if let Err(validation_errors) = result {
        for validation_error in validation_errors {
            error!(
                "Validation error: {}: `{}`",
                validation_error, validation_error.instance_path
            );
        }
    } else {
        debug!("Content passed validation");
    }
    Ok(())
}

#[instrument(skip(module_value))]
fn validate_module_schema(module_value: &'_ Value) -> Result<(), Box<dyn Error>> {
    info!("Validating module schema");
    let schema = reqwest::blocking::get(MODULE_SCHEMA)?.json::<Value>()?;
    let validator = JSONSchema::compile(&schema).expect("The module schema should be valid.");
    debug!("Loaded module schema");
    let result = validator.validate(module_value);
    if let Err(validation_errors) = result {
        for validation_error in validation_errors {
            error!(
                "Validation error: {} at `{}`",
                validation_error, validation_error.instance_path
            );
        }
    } else {
        info!("Basic module structure passed validation");
    }
    Ok(())
}
