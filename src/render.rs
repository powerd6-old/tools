extern crate clap;
extern crate fs;
extern crate module;
extern crate tracing;
extern crate tracing_subscriber;

use clap::Args;

use module::module::Module;
use module::module_type::{RenderingContent, RenderingFormat};
use module::Identifier;
use std::collections::HashMap;
use std::io::BufReader;
use std::{error::Error, ffi::OsString, fs::File, path::PathBuf};
use tracing::{debug, error, info, instrument};

/// Renders a module with a specific format
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct RenderArguments {
    /// The path to the module to be rendered
    #[arg(required = true)]
    source: PathBuf,
    /// The name of the output file, without extension
    #[arg(short = 'o', long = "output", default_value = "module")]
    output_file_name: OsString,
    /// The format that should be rendered
    #[arg(required = true)]
    format: String,
}

#[instrument]
pub fn run(
    RenderArguments {
        source,
        output_file_name,
        format,
    }: RenderArguments,
) -> Result<(), Box<dyn Error>> {
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
