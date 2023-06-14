use crate::{
    file_types::DetectFileTypes, path_utils::PathUtils, Entry, FileSystemError, RENDERING_DIRECTORY,
};

use std::{collections::HashMap, path::Path};

use serde_json::Value;

pub trait FileSystemData {
    fn try_get_data(&self) -> Result<Value, FileSystemError>;
}

impl FileSystemData for Path {
    fn try_get_data(&self) -> Result<Value, FileSystemError> {
        self.get_file_type()
            .map(|file_type| file_type.try_read_file(self))?
    }
}

impl FileSystemData for Entry {
    fn try_get_data(&self) -> Result<Value, FileSystemError> {
        match self {
            Entry::File(file) => file.try_get_data(),
            Entry::Directory {
                root_file,
                extra_files,
            } => match root_file.try_get_data() {
                Ok(mut root_data) => match root_data.as_object_mut() {
                    Some(root_data_map) => {
                        for extra_file in extra_files {
                            match extra_file.try_get_data() {
                                Ok(extra_data) => {
                                    root_data_map.insert(
                                        extra_file.get_name_without_extension(),
                                        extra_data,
                                    );
                                }
                                Err(e) => return Err(e),
                            }
                        }
                        Ok(root_data)
                    }
                    None => Err(FileSystemError::RootFileIsNotAnObject(
                        root_file.as_path().into(),
                    )),
                },
                Err(e) => Err(e),
            },
            Entry::RenderingDirectory {
                root_file,
                extra_files,
                rendering_files,
            } => match root_file.try_get_data() {
                Ok(mut root_data) => match root_data.as_object_mut() {
                    Some(root_data_map) => {
                        for extra_file in extra_files {
                            match extra_file.try_get_data() {
                                Ok(extra_data) => {
                                    root_data_map.insert(
                                        extra_file.get_name_without_extension(),
                                        extra_data,
                                    );
                                }
                                Err(e) => return Err(e),
                            }
                        }
                        let mut rendering_templates_data = HashMap::new();
                        for rendering_file in rendering_files {
                            match rendering_file.try_get_data() {
                                Ok(rendering_data) => {
                                    rendering_templates_data.insert(
                                        rendering_file.get_name_without_extension(),
                                        rendering_data,
                                    );
                                }
                                Err(e) => return Err(e),
                            }
                        }
                        root_data_map.insert(
                            RENDERING_DIRECTORY.to_string(),
                            serde_json::to_value(rendering_templates_data)
                                .expect("Map should be a valid Value"),
                        );
                        Ok(root_data)
                    }
                    None => Err(FileSystemError::RootFileIsNotAnObject(
                        root_file.as_path().into(),
                    )),
                },
                Err(e) => Err(e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use testdir::testdir;

    fn create_file(path: &PathBuf, contents: &str) -> PathBuf {
        std::fs::write(path, contents).expect("File was created correctly");
        path.to_path_buf()
    }
    fn create_directory(path: &PathBuf) -> PathBuf {
        std::fs::create_dir(path).expect("Directory was created correctly");
        path.to_path_buf()
    }

    #[test]
    fn single_file() {
        let dir = testdir!();
        let file = create_file(&dir.join("a.json"), r#"{"a":1}"#);
        assert_eq!(Entry::File(file).try_get_data().unwrap(), json!({"a":1}))
    }

    #[test]
    fn multiple_files() {
        let dir = testdir!();
        let root_file = create_file(&dir.join("a.json"), r#"{"a":1}"#);
        let extra_file_txt = create_file(&dir.join("text.txt"), "simple text");
        let extra_file_data = create_file(&dir.join("more_data.json"), r#"{"b":2}"#);
        assert_eq!(
            Entry::Directory {
                root_file,
                extra_files: vec![extra_file_txt, extra_file_data]
            }
            .try_get_data()
            .unwrap(),
            json!({"a":1,
                "text": "simple text",
            "more_data": {
                "b": 2
            }})
        )
    }

    #[test]
    fn multiple_files_and_rendering_directory() {
        let dir = testdir!();
        let root_file = create_file(&dir.join("a.json"), r#"{"a":1}"#);
        let extra_file_txt = create_file(&dir.join("text.txt"), "simple text");
        let extra_file_data = create_file(&dir.join("more_data.json"), r#"{"b":2}"#);
        let rendering = create_directory(&dir.join("rendering"));
        let rendering_txt = create_file(&rendering.join("txt.hjs"), "my render template");
        assert_eq!(
            Entry::RenderingDirectory {
                root_file,
                extra_files: vec![extra_file_txt, extra_file_data],
                rendering_files: vec![rendering_txt]
            }
            .try_get_data()
            .unwrap(),
            json!({"a":1,
                    "text": "simple text",
                "more_data": {
                    "b": 2
                },
                "rendering": {
                    "txt": "my render template"
                }
            })
        )
    }
}
