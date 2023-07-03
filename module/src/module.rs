use fs::file_system::FileSystem;
use fs_data::EntryData;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{info, instrument};
use url::Url;

use crate::{module_type::ModuleType, JsonMap, ModuleError};

/// A document that contains information for a powerd6 module.
///
/// While this object does not perform validation on it's own,
/// it creates an uniform structure to do so.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Module {
    /// The title of the module.
    title: String,
    /// The human-readable description of what the module contains.
    description: String,
    /// A hyperlink to the where the module is hosted.
    source: Url,
    /// A collection of types that are defined in this module.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<BTreeMap<String, ModuleType>>,
    /// A collection of contents defined in this module, the keys of the map are the unique identifiers of the content pieces.
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<BTreeMap<String, JsonMap>>,
}

impl Module {
    /// Extends the module with the provided types.
    ///
    /// If the Module does not already have types, then the provided types will simply be assigned.
    /// Otherwise, the provided types will be added, replacing the existing types when the identifiers match.
    fn extend_types(&mut self, mut extra_types: BTreeMap<String, ModuleType>) {
        if self.types.is_none() {
            self.types = Some(extra_types)
        } else {
            self.types
                .as_mut()
                .expect("Module should have types at this point")
                .append(&mut extra_types);
        }
    }

    /// Extends the module with the provided contents.
    ///
    /// If the Module does not already have contents, then the provided types will simply be assigned.
    /// Otherwise, the provided contents will be added, replacing the existing contents when the identifiers match.
    fn extend_contents(&mut self, mut extra_contents: BTreeMap<String, JsonMap>) {
        if self.contents.is_none() {
            self.contents = Some(extra_contents)
        } else {
            self.contents
                .as_mut()
                .expect("Module should have contents at this point")
                .append(&mut extra_contents);
        }
    }
}

impl TryFrom<FileSystem> for Module {
    type Error = ModuleError;

    #[instrument(skip(filesystem))]
    fn try_from(filesystem: FileSystem) -> Result<Module, ModuleError> {
        match filesystem.module.try_get_data() {
            Ok(module_data) => match serde_json::from_value::<Module>(module_data.clone()) {
                Ok(module) => {
                    let mut result = module;
                    if let Some(fs_types) = try_populate_types_from_filesystem(&filesystem)? {
                        result.extend_types(fs_types)
                    }
                    if let Some(fs_contents) = try_populate_contents_from_filesystem(&filesystem)? {
                        result.extend_contents(fs_contents)
                    }
                    Ok(result)
                }
                Err(_e) => Err(ModuleError::IncompatibleFieldType(module_data.into())),
            },
            Err(e) => Err(ModuleError::UnableToGetRequiredData(e.into())),
        }
    }
}

fn try_populate_types_from_filesystem(
    filesystem: &FileSystem,
) -> Result<Option<BTreeMap<String, ModuleType>>, ModuleError> {
    if let Some(fs_types) = &filesystem.types {
        let mut result: BTreeMap<String, ModuleType> = BTreeMap::new();
        info!("Loading types from file system");
        for type_entry in fs_types.entries.iter() {
            match &fs_types.get_identifier_for_entry(type_entry) {
                Some(identifier) => {
                    let module_type = ModuleType::try_from(type_entry.clone())?;
                    result.insert(identifier.to_string(), module_type);
                }
                None => {
                    return Err(ModuleError::InvalidIdentifier(Box::new(type_entry.clone())));
                }
            }
        }
        return Ok(Some(result));
    }
    Ok(None)
}

fn try_populate_contents_from_filesystem(
    filesystem: &FileSystem,
) -> Result<Option<BTreeMap<String, JsonMap>>, ModuleError> {
    if let Some(fs_contents) = &filesystem.contents {
        let mut result: BTreeMap<String, JsonMap> = BTreeMap::new();
        info!("Loading contents from file system");
        for content_entry in fs_contents.entries.iter() {
            match &fs_contents.get_identifier_for_entry(content_entry) {
                Some(identifier) => {
                    let content_data = content_entry
                        .try_get_data()
                        .map_err(|e| ModuleError::UnableToGetRequiredData(e.into()))?;
                    let module_content = serde_json::from_value(content_data.clone())
                        .or(Err(ModuleError::IncompatibleFieldType(content_data.into())))?;
                    result.insert(identifier.to_string(), module_content);
                }
                None => {
                    return Err(ModuleError::InvalidIdentifier(Box::new(
                        content_entry.clone(),
                    )));
                }
            }
        }
        return Ok(Some(result));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {

    use super::*;
    use fs::CONTENTS_DIRECTORY;
    use fs::TYPES_DIRECTORY;
    use path_utils::create_test_directory;
    use path_utils::create_test_file;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use testdir::testdir;

    #[test]
    fn works_with_only_mandatory_files() {
        let dir = testdir!();

        create_test_file(
            &dir.join("module.json"),
            r#"{
            "title": "My title",
            "description": "My description",
            "source": "https://powerd6.org"
        }"#,
        );

        assert_eq!(
            Module::try_from(FileSystem::try_from(dir).unwrap()).unwrap(),
            Module {
                title: "My title".to_string(),
                description: "My description".to_string(),
                source: Url::parse("https://powerd6.org").unwrap(),
                types: None,
                contents: None
            }
        )
    }

    #[test]
    fn types_are_populated_from_file_system_and_overwrite_types_from_module() {
        let dir = testdir!();

        create_test_file(
            &dir.join("module.json"),
            r#"{
                "title": "My title",
                "description": "My description",
                "source": "https://powerd6.org",
                "types": {
                    "a": {
                        "description": "my type"
                    }
                }
            }"#,
        );

        let types_directory = create_test_directory(&dir.join(TYPES_DIRECTORY));

        create_test_file(
            &types_directory.join("a.json"),
            r#"{
            "description": "my replaced type"
        }"#,
        );
        create_test_file(
            &types_directory.join("b.json"),
            r#"{
            "description": "my new type"
        }"#,
        );

        assert_eq!(
            Module::try_from(FileSystem::try_from(dir).unwrap()).unwrap(),
            Module {
                title: "My title".to_string(),
                description: "My description".to_string(),
                source: Url::parse("https://powerd6.org").unwrap(),
                types: Some(BTreeMap::from([
                    (
                        "a".to_string(),
                        ModuleType {
                            description: "my replaced type".to_string(),
                            schema: None,
                            rendering: None
                        }
                    ),
                    (
                        "b".to_string(),
                        ModuleType {
                            description: "my new type".to_string(),
                            schema: None,
                            rendering: None
                        }
                    )
                ])),
                contents: None
            }
        )
    }

    #[test]
    fn contents_are_populated_from_file_system_and_overwrite_contents_from_module() {
        let dir = testdir!();

        create_test_file(
            &dir.join("module.json"),
            r#"{
                "title": "My title",
                "description": "My description",
                "source": "https://powerd6.org",
                "contents": {
                    "a": {
                        "key": "value"
                    }
                }
            }"#,
        );

        let contents_directory = create_test_directory(&dir.join(CONTENTS_DIRECTORY));

        create_test_file(
            &contents_directory.join("a.json"),
            r#"{
            "key": "replaced value"
        }"#,
        );
        create_test_file(
            &contents_directory.join("b.json"),
            r#"{
            "key": "value"
        }"#,
        );

        assert_eq!(
            Module::try_from(FileSystem::try_from(dir).unwrap()).unwrap(),
            Module {
                title: "My title".to_string(),
                description: "My description".to_string(),
                source: Url::parse("https://powerd6.org").unwrap(),
                types: None,
                contents: Some(BTreeMap::from([
                    (
                        "a".to_string(),
                        BTreeMap::from([(
                            "key".to_string(),
                            Value::String("replaced value".to_string())
                        )])
                    ),
                    (
                        "b".to_string(),
                        BTreeMap::from([("key".to_string(), Value::String("value".to_string()))])
                    )
                ]))
            }
        )
    }
}
