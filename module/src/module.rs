use crate::{module_type::ModuleType, JsonObject, ModuleError, DESCRIPTION};

use super::Identifier;

use std::collections::HashMap;

use fs::{data::FileSystemData, Entry, FileSystem};

use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};
use url::Url;

const TITLE: &str = "title";
const SOURCE: &str = "source";
const TYPES: &str = "types";
const CONTENTS: &str = "contents";

/// A document that contains information detailing and explaining rules and/or content, meant to be used for rendering by powerd6.
/// This object does not perform validation into the values of each field and merely serves as a convenient way to manipulate already-built modules in rust.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// The title of the module.
    pub title: String,
    /// The human-readable description of what the module contains.
    pub description: String,
    /// A hyperlink to the where the module is hosted.
    pub source: Url,
    /// A collection of types that are defined in this module.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<HashMap<Identifier, ModuleType>>,
    /// A collection of contents defined in this module, the keys of the map are the unique identifiers of the content pieces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<Identifier, JsonObject>>,
}

impl Module {
    pub fn new(title: String, description: String, source: Url) -> Self {
        Module {
            title,
            description,
            source,
            types: None,
            content: None,
        }
    }
    pub fn with_types(mut self, types: HashMap<Identifier, ModuleType>) -> Self {
        self.types = Some(types);
        self
    }
    pub fn with_content(mut self, content: HashMap<Identifier, JsonObject>) -> Self {
        self.content = Some(content);
        self
    }
}

impl TryFrom<Entry> for Module {
    type Error = ModuleError;

    #[instrument]
    fn try_from(entry: Entry) -> Result<Self, Self::Error> {
        match entry.try_get_data() {
            Ok(value) => match value.as_object() {
                Some(data) => match (
                    data.get(TITLE).and_then(|r| r.as_str()),
                    data.get(DESCRIPTION).and_then(|r| r.as_str()),
                    data.get(SOURCE).and_then(|r| r.as_str()),
                ) {
                    (Some(title), Some(description), Some(source)) => match Url::parse(source) {
                        Ok(source_url) => {
                            let mut result =
                                Module::new(title.to_string(), description.to_string(), source_url);
                            if let Some(types) = data.get(TYPES).and_then(|t| t.as_object()) {
                                let mut types_result: HashMap<Identifier, ModuleType> =
                                    HashMap::new();
                                for (key, value) in types {
                                    match serde_json::from_value(value.clone()) {
                                        Ok(type_value) => {
                                            types_result.insert(key.to_owned().into(), type_value);
                                        }
                                        Err(e) => {
                                            return Err(ModuleError::UnableToBuildElement(e.into()))
                                        }
                                    }
                                }
                                result = result.with_types(types_result);
                            }
                            if let Some(contents) = data.get(CONTENTS).and_then(|t| t.as_object()) {
                                let mut contents_result: HashMap<Identifier, JsonObject> =
                                    HashMap::new();
                                for (key, value) in contents
                                    .into_iter()
                                    .filter_map(|(k, v)| Some(k).zip(v.as_object()))
                                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                                {
                                    contents_result
                                        .insert(key.into(), HashMap::from_iter(value.into_iter()));
                                }
                                result = result.with_content(contents_result);
                            }
                            Ok(result)
                        }
                        Err(e) => Err(ModuleError::UnableToBuildElement(e.into())),
                    },
                    _ => Err(ModuleError::MissingRequiredField(format!(
                        "{}, {}, or {}",
                        TITLE, DESCRIPTION, SOURCE
                    ))),
                },
                None => Err(ModuleError::NotAnObject),
            },
            Err(e) => Err(ModuleError::UnableToBuildElement(e.into())),
        }
    }
}

impl TryFrom<FileSystem> for Module {
    type Error = ModuleError;

    #[instrument]
    fn try_from(fs: FileSystem) -> Result<Self, Self::Error> {
        match TryInto::<Module>::try_into(fs.module) {
            Ok(mut module) => {
                debug!("Parsed module metadata");
                if let Some(fs_types) = fs.types {
                    debug!("Found types in file system");
                    let types_entries = fs_types.entries.clone().into_iter().filter_map(|e| {
                        e.get_id_from_nested_path(&fs_types)
                            .map(Identifier)
                            .zip(TryInto::<ModuleType>::try_into(e).ok())
                    });

                    match module.types.as_ref() {
                        None => module = module.with_types(types_entries.collect()),
                        Some(types_from_module) => {
                            debug!("Merging with types defined in base module");
                            let mut merged_types: HashMap<Identifier, ModuleType> = HashMap::new();
                            for (k, v) in types_from_module {
                                merged_types.insert(k.clone(), v.clone());
                            }
                            for (k, v) in types_entries {
                                merged_types.insert(k, v);
                            }
                            module = module.with_types(merged_types);
                        }
                    }
                }

                if let Some(fs_contents) = fs.contents {
                    debug!("Found contents in file system");
                    let fs_content_data = fs_contents.entries.clone().into_iter().filter_map(|e| {
                        e.get_id_from_nested_path(&fs_contents)
                            .map(Identifier)
                            .zip(e.try_get_data().ok())
                    });

                    let mut content_entries: HashMap<Identifier, JsonObject> = HashMap::new();
                    for (identifier, data) in fs_content_data {
                        match data.as_object() {
                            Some(fs_content_data_map) => {
                                content_entries.insert(
                                    identifier,
                                    JsonObject::from_iter(
                                        fs_content_data_map
                                            .into_iter()
                                            .map(|(k, v)| (k.clone(), v.clone())),
                                    ),
                                );
                            }
                            None => return Err(ModuleError::NotAnObject),
                        }
                    }

                    match module.content.as_ref() {
                        None => module = module.with_content(content_entries),
                        Some(content_from_module) => {
                            debug!("Merging with contents defined in base module");
                            let mut merged_content: HashMap<Identifier, JsonObject> =
                                HashMap::new();
                            for (k, v) in content_from_module {
                                merged_content.insert(k.clone(), v.clone());
                            }
                            for (k, v) in content_entries {
                                merged_content.insert(k.clone(), v.clone());
                            }
                            module = module.with_content(merged_content);
                        }
                    }
                }

                Ok(module)
            }
            Err(e) => Err(ModuleError::UnableToBuildElement(e.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::PathBuf};

    use super::*;
    use fs::{CONTENTS_DIRECTORY, MODULE, TYPES_DIRECTORY};
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use testdir::testdir;

    fn create_file(path: &PathBuf, contents: &str) -> PathBuf {
        std::fs::write(path, contents).expect("File could not be created");
        path.to_path_buf()
    }
    fn create_directory(path: &PathBuf) -> PathBuf {
        std::fs::create_dir(path).expect("Directory could not be created");
        path.to_path_buf()
    }

    #[test]
    fn only_required_fields() {
        let dir = testdir!();

        assert_eq!(
            Module::try_from(Entry::File(create_file(
                &dir.join("test.json"),
                r#"{
                    "title": "title",
                    "description": "description",
                    "source": "https://my.source"
                  }"#
            )))
            .unwrap(),
            Module::new(
                "title".to_string(),
                "description".to_string(),
                Url::parse("https://my.source").unwrap()
            )
        )
    }

    #[test]
    fn with_types() {
        let dir = testdir!();

        assert_eq!(
            Module::try_from(Entry::File(create_file(
                &dir.join("test.json"),
                r#"{
                    "title": "title",
                    "description": "description",
                    "source": "https://my.source",
                    "types": {
                      "a": {
                        "id": "a",
                        "description": "my description"
                      }
                    }
                  }"#
            )))
            .unwrap(),
            Module::new(
                "title".to_string(),
                "description".to_string(),
                Url::parse("https://my.source").unwrap()
            )
            .with_types(HashMap::from([(
                Identifier("a".to_string()),
                ModuleType::new("my description".to_string())
            )]))
        )
    }

    #[test]
    fn with_contents() {
        let dir = testdir!();

        assert_eq!(
            Module::try_from(Entry::File(create_file(
                &dir.join("test.json"),
                r#"{
                    "title": "title",
                    "description": "description",
                    "source": "https://my.source",
                    "contents": {
                      "a": {
                        "key": "value"
                      }
                    }
                  }"#
            )))
            .unwrap(),
            Module::new(
                "title".to_string(),
                "description".to_string(),
                Url::parse("https://my.source").unwrap()
            )
            .with_content(HashMap::from([(
                Identifier("a".to_string()),
                JsonObject::from([("key".to_string(), Value::String("value".to_string()))])
            )]))
        )
    }

    #[test]
    fn from_file_system() {
        let dir = testdir!();

        create_file(
            &dir.join(format!("{}.json", MODULE)),
            r#"{
                "title": "title",
                "description": "description",
                "source": "https://my.source",
                "types": {
                  "a": {
                    "id": "a",
                    "description": "my description"
                  }
                },
                "contents": {
                  "a": {
                    "key": "value"
                  }
                }
              }"#,
        );

        let types_dir = create_directory(&dir.join(TYPES_DIRECTORY));
        create_file(
            &types_dir.join("b.json"),
            r#"{
                "id": "b",
                "description": "my other description"
              }"#,
        );

        let contents_dir = create_directory(&dir.join(CONTENTS_DIRECTORY));
        create_file(
            &contents_dir.join("b.json"),
            r#"{
                "key": "value"
              }"#,
        );

        let file_system = FileSystem::try_from(dir)
            .expect("Could not create FileSystem from temporary test directory");

        let actual = Module::try_from(file_system).unwrap();
        let expected = Module::new(
            "title".to_string(),
            "description".to_string(),
            Url::parse("https://my.source").unwrap(),
        )
        .with_types(HashMap::from([
            (
                Identifier("a".to_string()),
                ModuleType::new("my description".to_string()),
            ),
            (
                Identifier("b".to_string()),
                ModuleType::new("my other description".to_string()),
            ),
        ]))
        .with_content(HashMap::from([
            (
                Identifier("a".to_string()),
                JsonObject::from([("key".to_string(), Value::String("value".to_string()))]),
            ),
            (
                Identifier("b".to_string()),
                JsonObject::from([("key".to_string(), Value::String("value".to_string()))]),
            ),
        ]));

        assert_eq!(actual.title, expected.title);
        assert_eq!(actual.description, expected.description);
        assert_eq!(actual.source, expected.source);
        let sorted_actual_types: BTreeMap<Identifier, ModuleType> = actual
            .types
            .expect("Resulting types do not exist")
            .into_iter()
            .collect();
        let sorted_expected_types: BTreeMap<Identifier, ModuleType> = expected
            .types
            .expect("Expected types do not exist")
            .into_iter()
            .collect();
        assert_eq!(sorted_actual_types, sorted_expected_types);

        let sorted_actual_content: BTreeMap<Identifier, JsonObject> = actual
            .content
            .expect("Resulting contents do not exist")
            .into_iter()
            .collect();
        let sorted_expected_content: BTreeMap<Identifier, JsonObject> = expected
            .content
            .expect("Expected contents do not exist")
            .into_iter()
            .collect();
        assert_eq!(sorted_actual_content, sorted_expected_content);
    }
}
