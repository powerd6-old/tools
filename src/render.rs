extern crate clap;
extern crate fs;
extern crate module;
extern crate thiserror;
extern crate tracing;
extern crate tracing_subscriber;

use clap::Args;

use self::thiserror::Error;
use module::module::Module;
use module::module_type::{RenderingContent, RenderingFormat};
use module::Identifier;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufReader;
use std::{ffi::OsString, fs::File, path::PathBuf};
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

/// The errors that can happen when rendering a Module
#[derive(Error, Debug)]
pub enum RenderError {
    #[error("missing types")]
    MissingTypes,
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
    let rendering_format = RenderingFormat::from(format);
    let rendering_templates = get_rendering_templates(module, rendering_format)?;
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

#[instrument(skip(module))]
fn get_rendering_templates(
    module: Module,
    format: RenderingFormat,
) -> Result<HashMap<Identifier, RenderingContent>, RenderError> {
    match module.types {
        Some(module_types) => {
            let mut result = HashMap::new();
            for (type_identifier, type_data) in module_types {
                match type_data.rendering {
                    Some(type_rendering) => {
                        if let Some(rendering_content) = type_rendering.get(&format) {
                            result.insert(type_identifier, rendering_content.clone());
                        } else {
                            error!(
                                "The type `{}` does not have a rendering template defined for `{:?}`.",
                                type_identifier, format
                            );
                        }
                    }
                    None => {
                        error!(
                            "The type `{}` does not have rendering templates defined.",
                            type_identifier
                        );
                    }
                }
            }
            Ok(result)
        }
        None => {
            error!("The module does not have any types");
            Err(RenderError::MissingTypes)
        }
    }
}
