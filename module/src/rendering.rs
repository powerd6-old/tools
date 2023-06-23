use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{module::Module, JsonObject};

/// The template to be used for the specified format.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RenderingContent(pub String);

impl From<String> for RenderingContent {
    fn from(value: String) -> Self {
        RenderingContent(value)
    }
}

/// The file format that the template corresponds to.
#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
pub struct RenderingFormat(pub String);

impl From<String> for RenderingFormat {
    fn from(value: String) -> Self {
        RenderingFormat(value)
    }
}

mod helpers;

impl RenderingContent {
    pub fn render(&self, content: &JsonObject, module: &Module) -> Result<String, Box<dyn Error>> {
        let mut handlebars = handlebars::Handlebars::new();
        handlebars = helpers::setup_helpers(handlebars);

        handlebars.register_template_string("template", self.0.clone())?;

        let render_data = JsonObject::from([
            ("self".to_string(), serde_json::to_value(content)?),
            ("module".to_string(), serde_json::to_value(module)?),
        ]);

        handlebars
            .render("template", &render_data)
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use url::Url;

    #[test]
    fn renders_non_templates() {
        let rendering_content = RenderingContent("Hello!".to_string());
        let content = JsonObject::new();
        let module = Module::new(
            "title".to_string(),
            "description".to_string(),
            Url::parse("https://my.source").unwrap(),
        );

        assert_eq!(
            rendering_content.render(&content, &module).unwrap(),
            "Hello!"
        );
    }

    #[test]
    fn renders_templates_with_data_from_content() {
        let rendering_content = RenderingContent("Name: {{self.name}}".to_string());
        let content = JsonObject::from([("name".to_string(), Value::String("john".to_string()))]);
        let module = Module::new(
            "title".to_string(),
            "description".to_string(),
            Url::parse("https://my.source").unwrap(),
        );

        assert_eq!(
            rendering_content.render(&content, &module).unwrap(),
            "Name: john"
        );
    }

    #[test]
    fn renders_templates_with_data_from_module() {
        let rendering_content = RenderingContent("Module title: {{module.title}}".to_string());
        let content = JsonObject::new();
        let module = Module::new(
            "title".to_string(),
            "description".to_string(),
            Url::parse("https://my.source").unwrap(),
        );

        assert_eq!(
            rendering_content.render(&content, &module).unwrap(),
            "Module title: title"
        );
    }
}
