use std::{collections::BTreeMap, f32::consts::E};

use fs::file_system::FileSystem;
use fs_data::EntryData;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use url::Url;

use crate::{module_type::ModuleType, ModuleError, DESCRIPTION, SOURCE, TITLE};

/// A document that contains information for a powerd6 module.
///
/// While this object does not perform validation on it's own,
/// it creates an uniform structure to do so.
#[derive(Serialize, Deserialize)]
pub struct Module {
    /// The title of the module.
    title: String,
    /// The human-readable description of what the module contains.
    description: String,
    /// A hyperlink to the where the module is hosted.
    source: Url,
    /// A collection of types that are defined in this module.
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<BTreeMap<String, ModuleType>>,
    /// A collection of contents defined in this module, the keys of the map are the unique identifiers of the content pieces.
    #[serde(skip_serializing_if = "Option::is_none")]
    contents: Option<BTreeMap<String, BTreeMap<String, Value>>>,
}

impl TryFrom<FileSystem> for Module {
    type Error = ModuleError;

    fn try_from(filesystem: FileSystem) -> Result<Module, ModuleError> {
        match filesystem.module.try_get_data() {
            Ok(fs_module_data) => match fs_module_data.as_object() {
                Some(module_data) => {
                    let title = get_data_field_as_str(module_data, TITLE)?;
                    let description = get_data_field_as_str(module_data, DESCRIPTION)?;
                    let source = get_data_field_as_str(module_data, SOURCE)?;
                    todo!()
                }
                None => Err(ModuleError::NotAnObject(fs_module_data)),
            },
            Err(e) => Err(ModuleError::UnableToGetRequiredData(e.into())),
        }
    }
}

fn get_data_field_as_str(data: &Map<String, Value>, field: &str) -> Result<String, ModuleError> {
    match data.get(field) {
        Some(value) => match value.as_str() {
            Some(str) => Ok(str.to_string()),
            None => Err(ModuleError::IncompatibleFieldType(value.clone().into())),
        },
        None => Err(ModuleError::MissingRequired(field.into())),
    }
}
