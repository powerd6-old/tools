use fs::{entry::Entry, entry_set::EntrySet, file_system::FileSystem};
use fs_data::EntryData;
use path_utils::identifier::IdentifierPaths;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::{info, instrument};
use url::Url;

use crate::{module_type::ModuleType, JsonMap, ModuleError};

/// A document that contains information for a powerd6 module.
///
/// While this object does not perform validation on it's own,
/// it creates an uniform structure to do so.
#[derive(Serialize, Deserialize, Debug)]
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

// TODO: Implement tests
impl TryFrom<FileSystem> for Module {
    type Error = ModuleError;

    fn try_from(filesystem: FileSystem) -> Result<Module, ModuleError> {
        match filesystem.module.try_get_data() {
            Ok(module_data) => match serde_json::from_value::<Module>(module_data.clone()) {
                Ok(module) => {
                    let mut result = module;
                    let fs_types = try_populate_types_from_filesystem(&filesystem)?;
                    result.extend_types(fs_types);
                    let fs_contents = try_populate_contents_from_filesystem(filesystem)?;
                    result.extend_contents(fs_contents);
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
) -> Result<BTreeMap<String, ModuleType>, ModuleError> {
    let mut result: BTreeMap<String, ModuleType> = BTreeMap::new();
    if let Some(fs_types) = &filesystem.types {
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
    }
    Ok(result)
}

fn try_populate_contents_from_filesystem(
    filesystem: FileSystem,
) -> Result<BTreeMap<String, JsonMap>, ModuleError> {
    let mut result: BTreeMap<String, JsonMap> = BTreeMap::new();
    if let Some(fs_contents) = filesystem.contents {
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
    }
    Ok(result)
}
