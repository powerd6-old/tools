use handlebars::Handlebars;
use module::module::Module;
use tracing::warn;

use crate::{
    renderer::{helpers::split_lines, ModuleRenderer},
    RenderingError,
};

pub trait RenderableModule {
    /// Uses the stored information to create a ModuleRenderer that holds all necessary information required to render content.
    fn get_renderer(&self) -> Result<ModuleRenderer, RenderingError>;
}

impl RenderableModule for Module {
    fn get_renderer(&self) -> Result<ModuleRenderer, RenderingError> {
        match &self.types {
            Some(types) => {
                let mut handlebars = Handlebars::new();

                // Register helpers
                handlebars.register_helper("splitLines", Box::new(split_lines::split_lines));

                // Compile templates for all types and formats
                for (type_key, module_type) in types {
                    if let Some(type_rendering) = &module_type.rendering {
                        for (format, template) in type_rendering {
                            handlebars
                                .register_template_string(
                                    &format!("{}_{}", type_key, format),
                                    template,
                                )
                                .map_err(|e| RenderingError::FailedToRegisterTemplate(e.into()))?;
                        }
                    } else {
                        warn!("Type `{}` does not have rendering templates", type_key);
                    }
                }

                Ok(ModuleRenderer {
                    module: self.clone(),
                    renderer: handlebars,
                })
            }
            None => Err(RenderingError::NoRenderableTypes),
        }
    }
}
