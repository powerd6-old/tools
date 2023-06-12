use jsonschema::JSONSchema;
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

// A identifier string. Must be unique within it's context.
pub struct Identifier(String);

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
    pub schema: Option<JSONSchema>,
    /// The rendering code for all the supported formats.
    pub rendering: Option<HashMap<RenderingFormat, RenderingContent>>,
}

/// The template to be used for the specified format.
pub struct RenderingContent(String);

/// The file format that the template corresponds to.
pub struct RenderingFormat(String);
