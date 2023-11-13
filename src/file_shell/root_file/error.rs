#[derive(Debug)]
pub enum FileSystemOperationError {
    NotFoundError(String),
    TooManyArgumentsError(String),
    LockError(String),
    UnableToOpenFile(String),
    NotDirError(String),
    ExistNameError(String),
    NotFileError(String),
    WriteError(String),
    DiskError(String),
    InitError(String),
    BadStructureError(String),
    LackArgumentsError(String),
    FileCreateError(String),
    DeleteError(String),
    DirItemError(String),
    PermissionError(String),
}
