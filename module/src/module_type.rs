use std::collections::BTreeMap;

use fs::entry::Entry;
use fs_data::EntryData;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::instrument;

use crate::ModuleError;

/// The representation of a powerd6 type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ModuleType {
    /// The human-readable description of what the type represents.
    pub description: String,
    /// The json-schema used to validate the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// The rendering code for all the supported formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<BTreeMap<String, String>>,
}

impl TryFrom<Entry> for ModuleType {
    type Error = ModuleError;

    #[instrument]
    fn try_from(entry: Entry) -> Result<ModuleType, ModuleError> {
        match entry.try_get_data() {
            Ok(entry_data) => match serde_json::from_value::<ModuleType>(entry_data.clone()) {
                Ok(result) => Ok(result),
                Err(_e) => Err(ModuleError::IncompatibleFieldType(entry_data.into())),
            },
            Err(e) => Err(ModuleError::UnableToGetRequiredData(e.into())),
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
    fn works_with_only_mandatory_fields() {
        let dir = testdir!();
        let file = create_test_file(
            &dir.join("a.json"),
            r#"{
            "description": "my type"
        }"#,
        );

        assert_eq!(
            ModuleType::try_from(Entry::File(file)).unwrap(),
            ModuleType {
                description: "my type".to_string(),
                schema: None,
                rendering: None
            }
        )
    }

    #[test]
    fn works_when_all_fields_are_present() {
        let dir = testdir!();
        let file = create_test_file(
            &dir.join("a.json"),
            r#"{
            "description": "my type",
            "schema": {
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string"
                    },
                    "age": {
                        "type": "integer",
                        "minimum": 0
                    }
                }
            },
            "rendering": {
                "txt": "my template"
            }
        }"#,
        );

        assert_eq!(
            ModuleType::try_from(Entry::File(file)).unwrap(),
            ModuleType {
                description: "my type".to_string(),
                schema: Some(json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string"
                        },
                        "age": {
                            "type": "integer",
                            "minimum": 0
                        }
                    }
                })),
                rendering: Some(BTreeMap::from([(
                    "txt".to_string(),
                    "my template".to_string()
                )]))
            }
        )
    }
}
