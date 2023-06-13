use super::FileTypeReader;

impl FileTypeReader for super::Yaml {
    fn read_file(
        &self,
        path: &std::path::Path,
    ) -> Result<serde_json::Value, crate::FileSystemError> {
        todo!()
    }
}
