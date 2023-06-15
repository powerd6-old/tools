use module_type::ModuleType;
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

pub mod module_type;
