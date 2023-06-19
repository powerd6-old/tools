extern crate clap;
extern crate fs;
extern crate module;

use std::{convert::TryFrom, error::Error, ffi::OsString, fs::File, io::Write, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use fs::FileSystem;
use module::module::Module;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match args.command {
        Commands::Build {
            source,
            output_file_name,
            output_type,
        } => {
            let file_system = FileSystem::try_from(source)?;
            let module = Module::try_from(file_system)?;
            let mut output_file = File::create(format!(
                "{}.json",
                output_file_name.to_str().expect("should be a valid string")
            ))?;
            let output_contents = match output_type {
                OutputType::Pretty => serde_json::to_string_pretty(&module)?,
                OutputType::Minimized => serde_json::to_string(&module)?,
            };
            write!(output_file, "{}", output_contents)?;
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
