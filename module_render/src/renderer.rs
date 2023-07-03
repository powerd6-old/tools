use std::collections::BTreeMap;

use handlebars::Handlebars;
use module::module::Module;
use serde_json::Value;

use crate::{RenderingError, TYPE_KEY};

/// The object responsible for rendering content of a module.
///
/// It holds the compiled templates and helpers for the module,
/// and exposes a simpler interface to render contents with a specific format.
pub struct ModuleRenderer<'handlebars> {
    pub module: Module,
    pub(crate) renderer: Handlebars<'handlebars>,
}

impl ModuleRenderer<'_> {
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

pub mod helpers;
