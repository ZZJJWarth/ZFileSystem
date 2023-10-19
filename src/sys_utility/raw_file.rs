use super::metadata::Metadata;

#[derive(Debug,Clone, Copy)]
pub enum FileType {
    File,
    Dir,
}

pub trait RawFile {
    fn get_type(&self) -> FileType;
    fn metadata(&self) -> Metadata;
    
}
