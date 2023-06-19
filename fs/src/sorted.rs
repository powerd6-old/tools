use super::EntrySet;

use super::Entry;

use super::FileSystem;

pub trait Sorted {
    /// Return a copy of the object, with all entries sorted
    fn sorted(&self) -> Self;
}

impl Sorted for FileSystem {
    fn sorted(&self) -> Self {
        let sorted_module = self.module.sorted();
        let sorted_types = self.types.as_ref().map(|t| t.sorted());
        let sorted_contents = self.contents.as_ref().map(|t| t.sorted());
        FileSystem {
            root_directory: self.root_directory.to_path_buf(),
            module: sorted_module,
            types: sorted_types,
            contents: sorted_contents,
        }
    }
}

impl Sorted for Entry {
    fn sorted(&self) -> Self {
        match self {
            Entry::File(f) => Entry::File(f.to_path_buf()),
            Entry::Directory {
                root_file,
                extra_files,
            } => {
                let mut sorted_extra_files = extra_files.clone();
                sorted_extra_files.sort();
                Entry::Directory {
                    root_file: root_file.to_path_buf(),
                    extra_files: sorted_extra_files,
                }
            }
            Entry::RenderingDirectory {
                root_file,
                extra_files,
                rendering_files,
            } => {
                let mut sorted_extra_files = extra_files.clone();
                sorted_extra_files.sort();
                let mut sorted_rendering_files = rendering_files.clone();
                sorted_rendering_files.sort();
                Entry::RenderingDirectory {
                    root_file: root_file.to_path_buf(),
                    extra_files: sorted_extra_files,
                    rendering_files: sorted_rendering_files,
                }
            }
        }
    }
}

impl Sorted for EntrySet {
    fn sorted(&self) -> Self {
        let mut sorted_entries: Vec<Entry> = self.entries.iter().map(|e| e.sorted()).collect();
        sorted_entries.sort();
        EntrySet {
            base_path: self.base_path.to_path_buf(),
            entries: sorted_entries,
        }
    }
}
