extern crate clap;
extern crate fs;
extern crate jsonschema;
extern crate module;
extern crate module_render;
extern crate serde_json;
extern crate thiserror;
extern crate tracing;
extern crate tracing_subscriber;

use std::error::Error;

use build::BuildArguments;
use clap::{Parser, Subcommand};

use render::RenderArguments;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use validate::ValidateArguments;

/// The entry point of the CLI execution.
fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing subscriber.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting up a default tracing subscriber should always succeed.");

    let args = Cli::parse();

    match args.command {
        Commands::Build(args) => build::run(args),
        Commands::Render(args) => render::run(args),
        Commands::Validate(args) => validate::run(args),
    }
}

/// A CLI to help build powerd6 modules.
#[derive(Debug, Parser)]
#[command(name = "powerd6_cli", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// The supported commands from the CLI.
#[derive(Debug, Subcommand)]
enum Commands {
    Build(BuildArguments),
    Render(RenderArguments),
    Validate(ValidateArguments),
}

/// Implements the [Build](crate::Commands::Build) command.
pub mod build;
/// Implements the [Render](crate::Commands::Render) command.
pub mod render;
/// Implements the [Validate](crate::Commands::Validate) command.
pub mod validate;
