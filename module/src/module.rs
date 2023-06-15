use crate::{module_type::ModuleType, JsonObject, ModuleError, DESCRIPTION};

use super::Identifier;

use std::{collections::HashMap, result};

use fs::{data::FileSystemData, Entry, FileSystem, CONTENTS_DIRECTORY};
use serde::de::value;
use url::Url;

const TITLE: &str = "title";
const SOURCE: &str = "source";
const TYPES: &str = "types";
const CONTENTS: &str = "contents";

/// A document that contains information detailing and explaining rules and/or content, meant to be used for rendering by powerd6.
/// This object does not perform validation into the values of each field and merely serves as a convenient way to manipulate already-built modules in rust.
#[derive(Debug, PartialEq)]
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
                                {
                                    contents_result.insert(
                                        key.to_owned().into(),
                                        HashMap::from_iter(value.to_owned().into_iter()),
                                    );
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

    fn try_from(value: FileSystem) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use testdir::testdir;

    fn create_file(path: &PathBuf, contents: &str) -> PathBuf {
        std::fs::write(path, contents).expect("File was created correctly");
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
}
