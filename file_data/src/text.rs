use crate::FileData;

pub struct TEXT;

impl FileData for TEXT {
    fn try_read_file(path: &std::path::Path) -> Result<serde_json::Value, crate::FileDataError> {
        todo!()
    }
}
