use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    io::{BufWriter, Seek},
    mem::transmute,
};

use crate::{
    file_shell::root_file::error::FileSystemOperationError, sys_utility::addr::addr::BlockAddr,
};

use super::super_block::SuperBlock;

#[derive(Clone, Copy, Debug)]
pub struct SuperPack {
    pub block_num: u32,           //磁盘总块数
    pub reserve_num: u32,         //保留给超级块的块数
    pub bitmap_num: u32,          //保留给bitmap的块数
    pub root_dir_addr: BlockAddr, //根目录的地址
    pub magic_num: u64,
    pub first_init: bool,
}

impl SuperPack {
    pub fn dump(self, path: &str) -> Result<(), FileSystemOperationError> {
        let f = File::options().write(true).open(path);
        let f = match f {
            Ok(s) => s,
            Err(_) => {
                return Err(FileSystemOperationError::DiskError(format!("未找到磁盘")));
            }
        };
        let mut bw = BufWriter::new(f);
        bw.seek(std::io::SeekFrom::Start(0));
        let buf = unsafe { transmute::<Self, [u8; SuperBlock::SUPER_BLOCK_SIZE as usize]>(self) };
        bw.write(&buf);
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, FileSystemOperationError> {
        let f = File::open(path);
        let f = match f {
            Ok(s) => s,
            Err(_) => {
                return Err(FileSystemOperationError::DiskError(format!("未找到磁盘")));
            }
        };
        let mut br = BufReader::with_capacity(SuperBlock::SUPER_BLOCK_SIZE as usize, f);
        br.seek(std::io::SeekFrom::Start(0));
        let mut buf: [u8; SuperBlock::SUPER_BLOCK_SIZE as usize] =
            [0; SuperBlock::SUPER_BLOCK_SIZE as usize];
        br.fill_buf();
        let bu = br.buffer();
        for i in 0..bu.len() {
            buf[i] = bu[i];
        }
        let temp = unsafe { transmute::<[u8; SuperBlock::SUPER_BLOCK_SIZE as usize], Self>(buf) };
        Ok(temp)
    }

    pub fn is_legal(&self) -> bool {
        self.magic_num == SuperBlock::MAGIC_FLAG
    }
}

#[cfg(test)]
#[test]
fn test_load() {}
