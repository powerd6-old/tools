use std::iter::Map;

use serde_json::Value;
use url::Url;

use crate::module_type::ModuleType;

/// A document that contains information detailing and explaining rules and/or content, meant to be used for rendering by powerd6.
pub struct Module {
    /// The title of the module.
    pub title: String,
    /// The human-readable description of what the module contains.
    pub description: String,
    /// A hyperlink to the where the module is hosted.
    pub source: Url,
    /// A collection of types that are defined in this module.
    pub types: Option<Map<String, ModuleType>>,
    /// A collection of contents defined in this module, the keys of the map are the unique identifiers of the content pieces.
    pub content: Option<Map<String, JsonObject>>,
}

pub type JsonObject = Map<String, Value>;
