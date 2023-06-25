use crate::FileData;

pub struct JSON;

impl FileData for JSON {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        todo!()
    }
}
