use crate::{FileDataError, FileTypeDataReader};
use std::fs::{self};

pub struct Text;

impl FileTypeDataReader for Text {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        fs::read_to_string(path)
            .map(serde_json::Value::String)
            .map_err(|e| FileDataError::UnableToOpenFile(path.into(), e.into()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use path_utils::create_test_file;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use testdir::testdir;

    #[test]
    fn fails_on_invalid_paths() {
        let dir = testdir!();

        let inexistent_file = &dir.join("fake.txt");

        assert!(Text::try_read_file(inexistent_file)
            .unwrap_err()
            .is_unable_to_open_file());
    }

    #[test]
    fn reads_sample_text_file() {
        let dir = testdir!();

        let sample_file = create_test_file(&dir.join("a.txt"), "abc");

        assert_eq!(
            Text::try_read_file(&sample_file).unwrap(),
            Value::String("abc".to_string())
        )
    }
}
