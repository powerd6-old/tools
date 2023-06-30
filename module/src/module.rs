use fs::{entry::Entry, entry_set::EntrySet, file_system::FileSystem};
use fs_data::EntryData;
use path_utils::identifier::IdentifierPaths;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use url::Url;

use crate::{module_type::ModuleType, JsonMap, ModuleError};

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
    contents: Option<BTreeMap<String, JsonMap>>,
}

impl Module {
    /// Inserts a new key-value pair for String,ModuleType into the Module.
    ///
    /// This initialized the types property of the module if it was empty.
    ///
    /// If another value was already present in the Module for this identifier, it's value is returned by this function.
    fn add_or_replace_type(
        &mut self,
        identifier: String,
        module_type: ModuleType,
    ) -> Option<ModuleType> {
        if self.types.is_none() {
            self.types = Some(BTreeMap::new())
        }
        self.types
            .as_mut()
            .expect("Module types should always exist at this point")
            .insert(identifier, module_type)
    }
    /// Inserts a new key-value pair for String,JsonMap into the Module.
    ///
    /// This initialized the contents property of the module if it was empty.
    ///
    /// If another value was already present in the Module for this identifier, it's value is returned by this function.
    fn add_or_replace_content(&mut self, identifier: String, content: JsonMap) -> Option<JsonMap> {
        if self.contents.is_none() {
            self.contents = Some(BTreeMap::new())
        }
        self.contents
            .as_mut()
            .expect("Module contents should always exist at this point")
            .insert(identifier, content)
    }
}

impl TryFrom<FileSystem> for Module {
    type Error = ModuleError;

    fn try_from(filesystem: FileSystem) -> Result<Module, ModuleError> {
        match filesystem.module.try_get_data() {
            Ok(module_data) => match serde_json::from_value::<Module>(module_data.clone()) {
                Ok(module) => {
                    let mut result = module;
                    if let Some(fs_types) = filesystem.types {
                        for type_entry in fs_types.entries.iter() {
                            let identifier = get_identifier_from(type_entry, &fs_types)?;
                            let module_type = ModuleType::try_from(type_entry.clone())?;
                            result.add_or_replace_type(identifier, module_type);
                        }
                    }
                    if let Some(fs_contents) = filesystem.contents {
                        for content_entry in fs_contents.entries.iter() {
                            let identifier = get_identifier_from(content_entry, &fs_contents)?;
                            let content_data = content_entry
                                .try_get_data()
                                .map_err(|e| ModuleError::UnableToGetRequiredData(e.into()))?;
                            let module_content = serde_json::from_value(content_data.clone())
                                .or(Err(ModuleError::IncompatibleFieldType(content_data.into())))?;
                            result.add_or_replace_content(identifier, module_content);
                        }
                    }
                    Ok(result)
                }
                Err(_e) => Err(ModuleError::IncompatibleFieldType(module_data.into())),
            },
            Err(e) => Err(ModuleError::UnableToGetRequiredData(e.into())),
        }
    }
}

// TODO: Move this function to another module?
fn get_identifier_from(type_entry: &Entry, fs_types: &EntrySet) -> Result<String, ModuleError> {
    let entry_path = match type_entry {
        Entry::File(file) => file,
        Entry::Directory {
            root_file,
            extra_files: _,
        } => root_file,
        Entry::RenderingDirectory {
            root_file,
            extra_files: _,
            rendering_files: _,
        } => root_file,
    };
    entry_path
        .clone()
        .get_id_from_path(&fs_types.base_path)
        .ok_or(ModuleError::InvalidIdentifier(
            entry_path.to_path_buf(),
            fs_types.base_path.clone(),
        ))
}
