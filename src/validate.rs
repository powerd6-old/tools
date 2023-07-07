extern crate clap;
extern crate fs;
extern crate module;
extern crate tracing;
extern crate tracing_subscriber;

use clap::Args;
use module::module::Module;
use std::io::BufReader;
use std::{error::Error, fs::File, path::PathBuf};
use tracing::{debug, info, instrument};

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
    debug!("Loaded module from file correctly: {:#?}", module);
    todo!()
}
