#[derive(Debug)]
pub enum FileSystemOperationError {
    NotFoundError(String),
}
