use std::collections::BTreeMap;

use fs::entry::Entry;
use fs_data::EntryData;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ModuleError;

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

impl TryFrom<Entry> for ModuleType {
    type Error = ModuleError;

    fn try_from(entry: Entry) -> Result<ModuleType, ModuleError> {
        match entry.try_get_data() {
            Ok(entry_data) => match serde_json::from_value::<ModuleType>(entry_data.clone()) {
                Ok(result) => Ok(result),
                Err(e) => Err(ModuleError::IncompatibleFieldType(entry_data.into())),
            },
            Err(e) => Err(ModuleError::UnableToGetRequiredData(e.into())),
        }
    }
}
