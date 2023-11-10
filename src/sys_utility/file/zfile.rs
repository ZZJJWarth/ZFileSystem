use crate::{
    file_shell::root_file::error::FileSystemOperationError,
    sys_utility::{addr::addr::BlockAddr, bitmap::block_bit_map::BlockBitmap},
};

use super::{
    raw_f::RawF,
    raw_file::{RawFile, ZFILE_SIZE},
};
#[derive(Debug)]
pub struct ZFile {
    raw: RawFile,
}

impl ZFile {
    pub fn new() -> ZFile {
        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        let entry = bit_map.get_free_block().unwrap();
        ZFile {
            raw: RawFile::new(super::raw_f::FileType::File, entry),
        }
    }

    pub fn open(ba: BlockAddr) -> ZFile {
        ZFile {
            raw: RawFile::open(ba).unwrap(),
        }
    }

    pub fn close(&mut self) {
        self.raw.close();
    }

    pub fn init_raw(raw: RawFile) {
        let mut f = ZFile { raw };
        f.close();
    }

    pub fn char_read(&self, offset: u32, size: u32) -> Vec<char> {
        let mut buf: Vec<u8> = vec![];
        self.raw.read(offset, &mut buf, size);
        let mut ans: Vec<char> = vec![];
        for i in buf {
            ans.push(i as char);
        }
        ans
    }

    pub fn char_write(&mut self, offset: u32, size: u32, buf: Vec<char>) -> Result<(), ()> {
        let mut b: Vec<u8> = vec![];
        for i in buf {
            b.push(i as u8);
        }
        self.raw.write(offset, &mut b, size);
        Ok(())
    }

    pub fn reduce(&mut self, size: u32) {
        self.raw.reduce(size);
    }

    pub fn get_block_entry(&self) -> BlockAddr {
        self.raw.get_block_entry()
    }

    pub fn del(&mut self) {
        self.raw.del();
    }

    pub fn cat(&self) -> String {
        let vec = self.char_read(0, self.raw.metadata().get_file_len() - ZFILE_SIZE as u32);
        vec.iter().collect()
    }

    pub fn write(&mut self, content: String) -> Result<usize, FileSystemOperationError> {
        let buf = content.as_bytes().to_vec();
        // println!("{:?}",buf);
        self.raw.add_write(&buf, content.len() as u32)
    }
}

impl Drop for ZFile {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
#[test]
fn test_zfile() {
    let mut zf = ZFile::open(BlockAddr { addr: 787 });
    println!("{:?}", zf);
    // zf.reduce(25);
    // zf.write(format!("hello"));
    // let input=vec!['h','e','l','l','o'];
    // zf.char_write(0, 5, input).unwrap();
    // zf.reduce(1);
    let output = zf.cat();
    println!("{}", output);
    // zf.close();
}
