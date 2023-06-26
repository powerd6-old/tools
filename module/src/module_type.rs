use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The representation of a powerd6 type.
#[derive(Serialize, Deserialize)]
pub struct ModuleType {
    /// The human-readable description of what the type represents.
    description: String,
    /// The json-schema used to validate the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Value>,
    /// The rendering code for all the supported formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    rendering: Option<BTreeMap<String, String>>,
}
