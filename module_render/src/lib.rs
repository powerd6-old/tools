use handlebars::Handlebars;
use module::module::Module;
use std::collections::HashMap;
use thiserror::Error;
use tracing::warn;

/// The errors that can happen when compiling or executing the rendering process for a Module.
#[derive(Error, Debug)]
pub enum RenderingError {
    #[error("no renderable types are present")]
    NoRenderableTypes,
}

pub trait RenderableModule {
    /// Uses the stored information to create a ModuleRendering that holds all necessary information required to render it.
    fn compile_rendering_templates(&self) -> Result<ModuleRendering, RenderingError>;
}

/// The object that holds the compiled rendering engine for a given module.
pub struct ModuleRendering<'a> {
    module: Module,
    renderer: Handlebars<'a>,
    templates: HashMap<FormatTypeKey, String>,
}

// impl ModuleRendering {
//     pub fn render(
//         &self,
//         content: _,
//         format: String,
//         type_id: String,
//     ) -> Result<String, RenderingError> {
//         todo!()
//     }
// }

type FormatTypeKey = (String, String);

impl RenderableModule for Module {
    fn compile_rendering_templates(&self) -> Result<ModuleRendering, RenderingError> {
        match self.types.as_ref() {
            Some(types) => {
                let mut templates: HashMap<FormatTypeKey, String> = HashMap::new();
                types
                    .iter()
                    .for_each(|(type_id, module_type)| match &module_type.rendering {
                        Some(module_type_rendering) => {
                            module_type_rendering.iter().for_each(|(format, template)| {
                                templates.insert(
                                    (format.to_string(), type_id.to_string()),
                                    template.to_string(),
                                );
                            })
                        }
                        None => {
                            warn!(
                                "The type `{}` does not have nay rendering template defined.",
                                type_id
                            );
                        }
                    });

                let handlebars = Handlebars::new();

                Ok(ModuleRendering {
                    module: self.clone(),
                    renderer: handlebars,
                    templates,
                })
            }
            None => Err(RenderingError::NoRenderableTypes),
        }
    }
}
