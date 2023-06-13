use super::FileTypeReader;

impl FileTypeReader for super::Json {
    fn read_file(
        &self,
        path: &std::path::Path,
    ) -> Result<serde_json::Value, crate::FileSystemError> {
        todo!()
    }
}
