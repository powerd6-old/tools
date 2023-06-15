use crate::{module_type::ModuleType, JsonObject, ModuleError, DESCRIPTION};

use super::Identifier;

use std::{collections::HashMap, result};

use fs::{data::FileSystemData, Entry};
use url::Url;

const TITLE: &str = "title";
const SOURCE: &str = "source";
const TYPES: &str = "types";

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
                Some(data) => match (data.get(TITLE), data.get(DESCRIPTION), data.get(SOURCE)) {
                    (Some(title), Some(description), Some(source)) => {
                        match Url::parse(&source.to_string()) {
                            Ok(source_url) => {
                                let mut result = Module::new(
                                    title.to_string(),
                                    description.to_string(),
                                    source_url,
                                );
                                if let Some(types) = data.get(TYPES) {
                                    match types.as_object() {
                                        Some(types_map) => {
                                            let mut types_result: HashMap<Identifier, ModuleType> =
                                                HashMap::new();
                                            for (key, value) in types_map {
                                                match serde_json::from_value(value.clone()) {
                                                    Ok(type_value) => {
                                                        types_result.insert(
                                                            key.to_owned().into(),
                                                            type_value,
                                                        );
                                                    }
                                                    Err(e) => {
                                                        return Err(
                                                            ModuleError::UnableToBuildElement(
                                                                e.into(),
                                                            ),
                                                        )
                                                    }
                                                }
                                            }
                                            result = result.with_types(types_result);
                                        }
                                        None => {
                                            return Err(ModuleError::UnableToBuildElement(
                                                ModuleError::NotAnObject.into(),
                                            ))
                                        }
                                    }
                                }
                                Ok(result)
                            }
                            Err(e) => Err(ModuleError::UnableToBuildElement(e.into())),
                        }
                    }
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
