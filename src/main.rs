extern crate clap;
extern crate fs;
extern crate module;
extern crate module_render;
extern crate tracing;
extern crate tracing_subscriber;

use std::error::Error;

use build::BuildArguments;
use clap::{Parser, Subcommand};

use render::RenderArguments;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting up a default tracing subscriber should always succeed");

    let args = Cli::parse();

    match args.command {
        Commands::Build(args) => build::run(args),
        Commands::Render(args) => render::run(args),
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
    Build(BuildArguments),
    Render(RenderArguments),
}

pub mod build;
pub mod render;
