use crate::{path_utils::PathUtils, Entry, EntrySet};

impl Entry {
    pub fn get_id_from_nested_path(&self, entry_set: &EntrySet) -> Option<String> {
        let entry_path = match self {
            Entry::File(f) => f,
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
        entry_path.get_id_from_path(&entry_set.base_path)
    }
}
