use std::collections::BTreeMap;

use handlebars::Handlebars;
use module::module::Module;
use serde_json::Value;

use crate::{RenderingError, TYPE_KEY};

/// The object responsible for rendering content of a [Module](module::module::Module).
///
/// It holds the compiled templates and helpers for the module,
/// and exposes a simpler interface to render contents with a specific format.
pub struct ModuleRenderer<'handlebars> {
    pub module: Module,
    pub(crate) renderer: Handlebars<'handlebars>,
}

impl ModuleRenderer<'_> {
    /// Renders a content with a registered template.
    ///
    /// It injects a `self` property with the content, and a `module` property
    /// with the entire module contents.
    pub fn render(
        &self,
        content: &BTreeMap<String, Value>,
        format: &str,
    ) -> Result<String, RenderingError> {
        match content.get(TYPE_KEY).and_then(|t| t.as_str()) {
            Some(type_key) => {
                let self_data = serde_json::to_value(content)
                    .expect("Content should always be a valid JSON Value");
                let module_data = serde_json::to_value(&self.module)
                    .expect("Module should always be a valid JSON Value");
                let data: BTreeMap<String, Value> = BTreeMap::from([
                    ("self".to_string(), self_data),
                    ("module".to_string(), module_data),
                ]);

                self.renderer
                    .render(&format!("{}_{}", type_key, format), &data)
                    .map_err(|e| RenderingError::FailedToRender(e.into()))
            }
            None => Err(RenderingError::ContentHasNoType(content.clone().into())),
        }
    }
}

/// A collection of [Handlebars helpers](https://handlebarsjs.com/guide/#custom-helpers).
pub mod helpers;
