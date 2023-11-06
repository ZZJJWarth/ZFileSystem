#[derive(Debug)]
pub enum FileSystemOperationError {
    NotFoundError(String),
    TooManyArgumentsError(String),
    LockError(String),
    UnableToOpenFile(String),
    NotDirError(String),
    ExistNameError(String),
}
