use super::block_servant::BlockServant;
use super::metadata::Metadata;
use super::raw_file::{FileType, RawFile};
struct ZFile {
    metadata: Metadata,
    file_name: String,
    block_servant: BlockServant,
    file_type: FileType,
}

impl RawFile for ZFile {
    fn get_type(&self) -> super::raw_file::FileType {
        self.file_type
    }

    fn metadata(&self) -> Metadata {
        self.metadata
    }
}

impl ZFile {
    pub fn read(offset: u32, buf: &mut Vec<u8>, size: u32) {
        todo!()
    }

    pub fn write(offset: u32, buf: &mut Vec<u8>, size: u32) {
        todo!()
    }
}
