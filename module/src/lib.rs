use fs::data::FileSystemData;
use fs::Entry;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use thiserror::Error;
use url::Url;

const DESCRIPTION: &str = "description";
const SCHEMA: &str = "schema";
const RENDERING: &str = "rendering";

// A identifier string. Must be unique within it's context.
pub struct Identifier(String);

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier(value)
    }
}

/// A document that contains information detailing and explaining rules and/or content, meant to be used for rendering by powerd6.
/// This object does not perform validation into the values of each field and merely serves as a convenient way to manipulate already-built modules in rust.
pub struct Module {
    /// The title of the module.
    pub title: String,
    /// The human-readable description of what the module contains.
    pub description: String,
    /// A hyperlink to the where the module is hosted.
    pub source: Url,
    /// A collection of types that are defined in this module.
    pub types: Option<HashMap<Identifier, ModuleType>>,
    /// A collection of contents defined in this module, the keys of the map are the unique identifiers of the content pieces.
    pub content: Option<HashMap<Identifier, JsonObject>>,
}

/// A generic object that contains data
pub type JsonObject = HashMap<String, Value>;

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

/// The errors that can happen when constructing a Module
#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("unable to build element")]
    UnableToBuildElement(#[from] Box<dyn Error>),
    #[error("received value is not an object")]
    NotAnObject,
    #[error("missing required field `{0}`")]
    MissingRequiredField(String),
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
