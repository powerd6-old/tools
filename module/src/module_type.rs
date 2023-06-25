use crate::rendering::RenderingContent;
use crate::rendering::RenderingFormat;

use super::DESCRIPTION;

use super::ModuleError;

use fs::data::FileSystemData;
use fs::Entry;
use serde::Deserialize;
use serde::Serialize;
use tracing::debug;
use tracing::instrument;

use std::collections::HashMap;

use serde_json::Value;

const SCHEMA: &str = "schema";
const RENDERING: &str = "rendering";

/// The aggregation of properties, their value-types and their rendering rules.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModuleType {
    /// The human-readable description of what the type represents.
    pub description: String,
    /// The json-schema used to validate the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// The rendering code for all the supported formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<HashMap<RenderingFormat, RenderingContent>>,
}

impl ModuleType {
    pub fn new(description: String) -> Self {
        ModuleType {
            description,
            schema: None,
            rendering: None,
        }
    }
    pub fn with_schema(mut self, schema: Value) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn with_rendering(mut self, rendering: HashMap<RenderingFormat, RenderingContent>) -> Self {
        self.rendering = Some(rendering);
        self
    }
}

impl TryFrom<Entry> for ModuleType {
    type Error = ModuleError;

    #[instrument(skip(entry))]
    fn try_from(entry: Entry) -> Result<Self, Self::Error> {
        debug!("Creating ModuleType from Entry");
        match entry.try_get_data() {
            Ok(value) => match value.as_object() {
                Some(data) => match data.get(DESCRIPTION).and_then(|d| d.as_str()) {
                    Some(description) => {
                        let mut result = ModuleType::new(description.to_string());
                        if let Some(schema) = data.get(SCHEMA) {
                            result = result.with_schema(schema.clone());
                        }
                        if let Some(rendering) = data.get(RENDERING).and_then(|r| r.as_object()) {
                            let mut rendering_result: HashMap<RenderingFormat, RenderingContent> =
                                HashMap::new();
                            for (key, value) in rendering
                                .into_iter()
                                .filter_map(|(k, v)| Some(k).zip(v.as_str()))
                            {
                                rendering_result
                                    .insert(key.to_owned().into(), value.to_string().into());
                            }

                            result = result.with_rendering(rendering_result)
                        }
                        Ok(result)
                    }
                    None => Err(ModuleError::MissingRequiredField(DESCRIPTION.to_string())),
                },
                None => Err(ModuleError::NotAnObject),
            },
            Err(e) => Err(ModuleError::UnableToBuildElement(e.into())),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use path_utils::create_test_file;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use testdir::testdir;

    #[test]
    fn only_required_fields() {
        let dir = testdir!();

        assert_eq!(
            ModuleType::try_from(Entry::File(create_test_file(
                &dir.join("test.json"),
                r#"{"description": "this is a description"}"#
            )))
            .unwrap(),
            ModuleType::new("this is a description".to_string())
        )
    }

    #[test]
    fn with_schema() {
        let dir = testdir!();

        assert_eq!(
            ModuleType::try_from(Entry::File(create_test_file(
                &dir.join("test.json"),
                r#"{
                    "description": "this is a description",
                    "schema": {
                      "$schema": "https://json-schema.org/draft/2020-12/schema",
                      "title": "Person",
                      "type": "object",
                      "properties": {
                        "fullName": {
                          "type": "string",
                          "description": "The person's name."
                        }
                      }
                    }
                  }"#
            )))
            .unwrap(),
            ModuleType::new("this is a description".to_string()).with_schema(json!({
              "$schema": "https://json-schema.org/draft/2020-12/schema",
              "title": "Person",
              "type": "object",
              "properties": {
                "fullName": {
                  "type": "string",
                  "description": "The person's name."
                }
              }
            }))
        )
    }

    #[test]
    fn with_rendering() {
        let dir = testdir!();

        assert_eq!(
            ModuleType::try_from(Entry::File(create_test_file(
                &dir.join("test.json"),
                r#"{
                    "description": "this is a description",
                    "rendering": {
                      "txt": "this is my txt template"
                    }
                  }"#
            )))
            .unwrap(),
            ModuleType::new("this is a description".to_string()).with_rendering(HashMap::from([(
                RenderingFormat("txt".to_string()),
                RenderingContent("this is my txt template".to_string())
            )]))
        )
    }
}
