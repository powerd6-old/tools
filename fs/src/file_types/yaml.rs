use std::{fs::File, io::BufReader};

use crate::FileSystemError;

use super::FileTypeReader;

impl FileTypeReader for super::Yaml {
    fn read_file(
        &self,
        path: &std::path::Path,
    ) -> Result<serde_json::Value, crate::FileSystemError> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_yaml::from_reader(reader)
                    .map_err(|e| FileSystemError::UnableToOpenFile(Box::new(e)))
            }
            Err(e) => Err(FileSystemError::UnableToOpenFile(Box::new(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_types::Yaml;

    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use testdir::testdir;

    #[test]
    fn it_parses_yaml_into_json_value() {
        let dir = testdir!();
        let file = &dir.join("a.yaml");
        std::fs::write(
            file,
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
        )
        .expect("File was created correctly");

        assert_eq!(
            Yaml.read_file(file).unwrap(),
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
        )
    }
}
