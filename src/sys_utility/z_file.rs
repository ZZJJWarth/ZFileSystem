use super::metadata::Metadata;
use super::block_servant::BlockServant;
use super::raw_file::{RawFile, FileType};
struct ZFile{
    metadata:Metadata,
    file_name:String,
    block_servant:BlockServant,
    file_type:FileType,
}

impl RawFile for ZFile{
    fn get_type(&self)->super::raw_file::FileType {
        self.file_type
    }

    fn metadata(&self)->Metadata {
        self.metadata
    }
}