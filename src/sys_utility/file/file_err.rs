pub enum DirDeleleError {
    NotFoundError(String),
    NotShortItemError(String),
    NotEmptyDirError(String),
}
