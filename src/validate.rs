use clap::Args;
use jsonschema::JSONSchema;
use module::module::Module;
use serde_json::Value;
use std::io::BufReader;
use std::{error::Error, fs::File, path::PathBuf};
use tracing::{debug, error, info, instrument};

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
    let module_schema = reqwest::blocking::get(MODULE_SCHEMA)?.json::<Value>()?;
    let module_schema_validator =
        JSONSchema::compile(&module_schema).expect("The module schema should be valid.");
    debug!("Loaded module schema");
    let result = module_schema_validator.validate(&module_value);
    if let Err(errors) = result {
        for error in errors {
            error!("Validation error: {}", error);
        }
    }
    todo!()
}

const MODULE_SCHEMA: &str = "https://specification.powerd6.org/schemas/module.json";
const TYPE_SCHEMA: &str = "https://specification.powerd6.org/schemas/type.json";
const CONTENT_SCHEMA: &str = "https://specification.powerd6.org/schemas/content.json";
