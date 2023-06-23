extern crate clap;
extern crate fs;
extern crate module;
extern crate tracing;
extern crate tracing_subscriber;

use std::{
    collections::HashMap,
    convert::TryFrom,
    error::Error,
    ffi::OsString,
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

use clap::{Parser, Subcommand, ValueEnum};
use fs::FileSystem;
use module::{
    module::Module,
    module_type::{RenderingContent, RenderingFormat},
    Identifier,
};
use tracing::{debug, error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting the default subscriber failed");

    let args = Cli::parse();

    match args.command {
        Commands::Build {
            source,
            output_file_name,
            output_type,
        } => {
            info!("Starting to build the module");
            let file_system = FileSystem::try_from(source)?;
            debug!("Source directory was parsed correctly");
            let module = Module::try_from(file_system)?;
            info!("Module was created from source directory");
            let mut output_file = File::create(format!(
                "{}.json",
                output_file_name
                    .to_str()
                    .expect("output file name is not a valid string")
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
        Commands::Render {
            source,
            output_file_name,
            format,
        } => {
            info!("Starting to render the module");
            let file = File::open(source)?;
            let reader = BufReader::new(file);
            let module: Module = serde_json::from_reader(reader)?;
            debug!("Loaded module from file correctly");
            // TODO: check all types have the chosen template format
            let mut rendering_templates: HashMap<Identifier, RenderingContent> = HashMap::new();
            for (type_identifier, type_data) in module
                .types
                .expect("The module does not have types defined")
            {
                if let Some(rendering_content) = type_data
                    .rendering
                    .expect("The type does not have a rendering template for this format")
                    .get(&RenderingFormat::from(format.clone()))
                {
                    rendering_templates.insert(type_identifier, rendering_content.clone());
                } else {
                    error!(
                        "The type `{}` does not have a rendering template defined for `{}`.",
                        type_identifier, format
                    );
                }
            }
            debug!(
                "Found rendering templates for types: {:?}",
                rendering_templates.keys()
            );
            // TODO: Compile the rendering templates for each type
            // TODO: For each piece of content, render it with the correct template
            // TODO: Save all rendered contents into output
            info!("Done!");
            Ok(())
        }
    }
}

/// A CLI to help build powerd6 modules
#[derive(Debug, Parser)]
#[command(name = "powerd6_cli", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Builds a module from files in your computer
    #[command(arg_required_else_help = true)]
    Build {
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
    },
    /// Renders a module with a specific format
    #[command(arg_required_else_help = true)]
    Render {
        /// The path to the module to be rendered
        #[arg(required = true)]
        source: PathBuf,
        /// The name of the output file, without extension
        #[arg(short = 'o', long = "output", default_value = "module")]
        output_file_name: OsString,
        /// The format that should be rendered
        #[arg(required = true)]
        format: String,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum OutputType {
    Pretty,
    Minimized,
}
