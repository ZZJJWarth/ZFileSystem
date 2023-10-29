use super::metadata::Metadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Dir,
}

pub trait RawF {
    fn get_type(&self) -> FileType;
    fn metadata(&self) -> Metadata;
}
