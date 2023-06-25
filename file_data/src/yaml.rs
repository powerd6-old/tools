use crate::FileData;

pub struct YAML;

impl FileData for YAML {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        todo!()
    }
}
