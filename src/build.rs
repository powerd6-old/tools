extern crate clap;
extern crate fs;
extern crate module;
extern crate tracing;
extern crate tracing_subscriber;

use clap::{Args, ValueEnum};
use fs::file_system::FileSystem;
use module::module::Module;
use std::io::Write;
use std::{convert::TryFrom, error::Error, ffi::OsString, fs::File, path::PathBuf};
use tracing::{debug, info, instrument};

/// Builds a module from files in your computer
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct BuildArguments {
    /// The path to the directory that will be built
    #[arg(required = true)]
    source: PathBuf,
    /// The name of the output file, without extension
    #[arg(short = 'o', long = "output", default_value = "module")]
    output_file_name: OsString,
    /// What type of output should be generated
    #[arg(
            short = 't',
            long = "type",            
            default_value_t = OutputType::Pretty,
            value_enum
        )]
    output_type: OutputType,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum OutputType {
    Pretty,
    Minimized,
}

#[instrument]
pub fn run(
    BuildArguments {
        source,
        output_file_name,
        output_type,
    }: BuildArguments,
) -> Result<(), Box<dyn Error>> {
    info!("Starting to build the module");
    let file_system = FileSystem::try_from(source)?;
    debug!("Source directory was parsed correctly: {:#?}", file_system);
    let module = Module::try_from(file_system)?;
    info!("Module was created from source directory: {:#?}", module);
    let mut output_file = File::create(format!(
        "{}.json",
        output_file_name
            .to_str()
            .expect("The output file name should be a valid UTF-8 String")
    ))?;
    debug!("About to write module as {:?}", output_type);
    let output_contents = match output_type {
        OutputType::Pretty => serde_json::to_string_pretty(&module)?,
        OutputType::Minimized => serde_json::to_string(&module)?,
    };
    write!(output_file, "{}", output_contents)?;
    info!("Done!");
    Ok(())
}
