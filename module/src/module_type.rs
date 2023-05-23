use jsonschema::JSONSchema;
use std::iter::Map;

/// The aggregation of properties, their value-types and their rendering rules.
pub struct ModuleType {
    /// The unique identifier of the type.
    pub id: String,
    /// The human-readable description of what the type represents.
    pub description: String,
    /// The json-schema used to validate the type.
    pub schema: Option<JSONSchema>,
    /// The rendering code for all the supported formats.
    pub rendering: Option<Map<String, RenderingContent>>,
}

/// The template to be used for the specified format
pub struct RenderingContent(String);