use crate::{FileDataError, FileTypeDataReader};
use std::{fs::File, io::BufReader};

pub struct YAML;

impl FileTypeDataReader for YAML {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_yaml::from_reader(reader)
                    .map_err(|e| FileDataError::InvalidFileContents(e.into()))
            }
            Err(e) => Err(FileDataError::UnableToOpenFile(e.into())),
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
    fn valid_yaml_becomes_valid_json() {
        let dir = testdir!();

        let yaml = create_test_file(
            &dir.join("file.yaml"),
            r#"
        # comment
        key: value
        integerValue: 1
        floatingValue: 1
        stringValue: abc
        booleanValue: true
        multilineString: |
            Line1
            line2
        array:
            - One
            - two
            - Three
        map:
            a: 1
            b: 2
                    "#,
        );

        assert_eq!(
            YAML::try_read_file(&yaml).unwrap(),
            json!({
              "key": "value",
              "integerValue": 1,
              "floatingValue": 1,
              "stringValue": "abc",
              "booleanValue": true,
              "multilineString": "Line1\nline2\n",
              "array": [
                "One",
                "two",
                "Three"
              ],
              "map": {
                "a": 1,
                "b": 2
              }
            })
        );
    }
}
