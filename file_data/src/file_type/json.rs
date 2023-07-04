use crate::{FileDataError, FileTypeDataReader};
use std::{fs::File, io::BufReader};

pub struct Json;

impl FileTypeDataReader for Json {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader)
                    .map_err(|e| FileDataError::InvalidFileContents(path.into(), e.into()))
            }
            Err(e) => Err(FileDataError::UnableToOpenFile(path.into(), e.into())),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use path_utils::create_test_file;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use testdir::testdir;

    #[test]
    fn fails_on_invalid_paths() {
        let dir = testdir!();

        let inexistent_file = &dir.join("fake.json");

        assert!(Json::try_read_file(inexistent_file)
            .unwrap_err()
            .is_unable_to_open_file());
    }

    #[test]
    fn fails_on_invalid_json() {
        let dir = testdir!();

        let invalid_file = create_test_file(&dir.join("a.json"), r#" "#);

        assert!(Json::try_read_file(&invalid_file)
            .unwrap_err()
            .is_invalid_file_contents());
    }

    #[test]
    fn reads_sample_json() {
        let dir = testdir!();

        let sample_file = create_test_file(
            &dir.join("a.json"),
            r#"{
            "a": 1
        }"#,
        );

        assert_eq!(
            Json::try_read_file(&sample_file).unwrap(),
            json!({
                "a": 1
            })
        )
    }
}
