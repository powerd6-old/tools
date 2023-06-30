extern crate clap;
extern crate fs;
extern crate module;
extern crate thiserror;
extern crate tracing;
extern crate tracing_subscriber;

use clap::Args;
use module::rendering::{RenderingContent, RenderingFormat};

use self::thiserror::Error;
use module::module::Module;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufReader;
use std::io::Write;
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
    #[error("found no types in the module")]
    MissingTypes,
    #[error("found no contents in the module")]
    MissingContents,
    #[error("unable to render template")]
    RenderingError(#[from] Box<dyn Error>),
}

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
    debug!("Loaded module from file correctly: {:#?}", module);
    let rendering_format = RenderingFormat::from(format.clone());
    let rendering_templates = get_rendering_templates(&module, &rendering_format)?;
    debug!(
        "Found rendering templates for types: {:?}",
        rendering_templates.keys()
    );
    // TODO: For each piece of content, render it with the correct template
    let rendered_module = render_contents(&module, &rendering_templates)?;
    debug!("Rendered contents to string");
    let mut output_file = File::create(format!(
        "{}.{}",
        output_file_name
            .to_str()
            .expect("The output file name should be a valid UTF-8 String"),
        format
    ))?;
    write!(output_file, "{}", rendered_module)?;
    info!("Done!");
    Ok(())
}

#[instrument(skip(module))]
fn get_rendering_templates(
    module: &Module,
    format: &RenderingFormat,
) -> Result<HashMap<Identifier, RenderingContent>, RenderError> {
    match &module.types {
        Some(module_types) => {
            let mut result = HashMap::new();
            for (type_identifier, type_data) in module_types {
                match &type_data.rendering {
                    Some(type_rendering) => {
                        if let Some(rendering_content) = type_rendering.get(format) {
                            result.insert(type_identifier.clone(), rendering_content.clone());
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
fn render_contents(
    module: &Module,
    templates: &HashMap<Identifier, RenderingContent>,
) -> Result<String, RenderError> {
    match &module.content {
        Some(module_contents) => {
            let mut result = String::new();
            for (content_key, content) in module_contents {
                match content.get("type") {
                    Some(content_type) => {
                        let type_identifier: Identifier = content_type
                            .as_str()
                            .expect("The content type should be a valid UTF-8 String")
                            .to_string()
                            .into();
                        match templates.get(&type_identifier) {
                            Some(template) => match template.render(content, module) {
                                Ok(rendered_content) => {
                                    result += &rendered_content;
                                    result += "\n";
                                }
                                Err(e) => {
                                    error!("Failed to render template for content {}", content_key);
                                    return Err(RenderError::RenderingError(e));
                                }
                            },
                            None => {
                                error!(
                                    "There is no rendering template for type `{}`",
                                    type_identifier
                                )
                            }
                        }
                    }
                    None => {
                        error!(
                            "The content `{}` does not have a `type` property",
                            content_key
                        );
                    }
                }
            }
            Ok(result)
        }
        None => {
            error!("The module does not have any contents");
            Err(RenderError::MissingContents)
        }
    }
}
