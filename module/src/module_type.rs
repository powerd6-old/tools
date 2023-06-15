use super::RENDERING;

use super::SCHEMA;

use super::DESCRIPTION;

use super::ModuleError;

use fs::data::FileSystemData;
use fs::Entry;

use std::collections::HashMap;

use serde_json::Value;

use super::Identifier;

/// The aggregation of properties, their value-types and their rendering rules.
pub struct ModuleType {
    /// The unique identifier of the type.
    pub id: Identifier,
    /// The human-readable description of what the type represents.
    pub description: String,
    /// The json-schema used to validate the type.
    pub schema: Option<Value>,
    /// The rendering code for all the supported formats.
    pub rendering: Option<HashMap<RenderingFormat, RenderingContent>>,
}

impl ModuleType {
    pub fn new(id: Identifier, description: String) -> Self {
        ModuleType {
            id,
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

/// The template to be used for the specified format.
pub struct RenderingContent(String);

impl From<String> for RenderingContent {
    fn from(value: String) -> Self {
        RenderingContent(value)
    }
}

/// The file format that the template corresponds to.
#[derive(Eq, Hash, PartialEq)]
pub struct RenderingFormat(String);

impl From<String> for RenderingFormat {
    fn from(value: String) -> Self {
        RenderingFormat(value)
    }
}

impl TryFrom<Entry> for ModuleType {
    type Error = ModuleError;

    fn try_from(entry: Entry) -> Result<Self, Self::Error> {
        match entry.try_get_data() {
            Ok(value) => match value.as_object() {
                Some(data) => match data.get(DESCRIPTION) {
                    Some(description) => {
                        let mut result = ModuleType::new(
                            entry.get_id_from_path().into(),
                            description.to_string(),
                        );
                        if let Some(schema) = data.get(SCHEMA) {
                            result = result.with_schema(schema.clone());
                        }
                        if let Some(rendering) = data.get(RENDERING) {
                            match rendering.as_object() {
                                Some(rendering_map) => {
                                    let mut rendering_result: HashMap<
                                        RenderingFormat,
                                        RenderingContent,
                                    > = HashMap::new();
                                    for (key, value) in rendering_map {
                                        rendering_result.insert(
                                            key.to_owned().into(),
                                            value.to_string().into(),
                                        );
                                    }
                                    result = result.with_rendering(rendering_result)
                                }
                                None => {
                                    return Err(ModuleError::UnableToBuildElement(
                                        ModuleError::NotAnObject.into(),
                                    ))
                                }
                            }
                        }
                        Ok(result)
                    }
                    None => Err(ModuleError::MissingRequiredField(DESCRIPTION.to_string())),
                },
                None => Err(ModuleError::MissingRequiredField(DESCRIPTION.to_string())),
            },
            Err(e) => Err(ModuleError::UnableToBuildElement(e.into())),
        }
    }
}
