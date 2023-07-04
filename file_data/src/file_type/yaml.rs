use crate::{FileDataError, FileTypeDataReader};
use std::{fs::File, io::BufReader};

pub struct Yaml;

impl FileTypeDataReader for Yaml {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_yaml::from_reader(reader)
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

        let inexistent_file = &dir.join("fake.yaml");

        assert!(Yaml::try_read_file(inexistent_file)
            .unwrap_err()
            .is_unable_to_open_file());
    }

    #[test]
    fn fails_on_invalid_yaml() {
        let dir = testdir!();

        // YAML does not allow duplicate keys.
        let invalid_file = create_test_file(
            &dir.join("a.yaml"),
            r#"
        a:
  a: 1
  a: 2"#,
        );

        assert!(Yaml::try_read_file(&invalid_file)
            .unwrap_err()
            .is_invalid_file_contents());
    }

    #[test]
    fn valid_yaml_becomes_valid_json() {
        let dir = testdir!();

        let sample_file = create_test_file(
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
            Yaml::try_read_file(&sample_file).unwrap(),
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
