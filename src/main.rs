extern crate clap;
extern crate fs;
extern crate module;
extern crate tracing;
extern crate tracing_subscriber;

use std::{convert::TryFrom, error::Error, ffi::OsString, fs::File, io::Write, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use fs::FileSystem;
use module::module::Module;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting the default subscriber failed");

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
    }
}

/// A CLI to help build powerd6 modules
#[derive(Debug, Parser)]
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
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum OutputType {
    Pretty,
    Minimized,
}
