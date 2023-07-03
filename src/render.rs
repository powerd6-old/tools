extern crate clap;
extern crate fs;
extern crate module;
extern crate module_render;
extern crate thiserror;
extern crate tracing;
extern crate tracing_subscriber;

use clap::Args;
use module_render::module::RenderableModule;
use module_render::renderer::ModuleRenderer;

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
    #[error("found no contents in the module")]
    MissingContents,
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
    let module_renderer = module.get_renderer()?;
    debug!("Compiled the rendering for module");
    let rendered_module = render_contents(&module_renderer, &format)?;
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

#[instrument(skip(renderer))]
fn render_contents(renderer: &ModuleRenderer, format: &String) -> Result<String, Box<dyn Error>> {
    match &renderer.module.contents {
        Some(contents) => {
            let mut result = String::new();
            for value in contents.values() {
                result += &renderer.render(value, format)?;
                result += "\n";
            }
            todo!()
        }
        None => {
            error!("No contents were present in the module");
            Err(Box::new(RenderError::MissingContents))
        }
    }
}
