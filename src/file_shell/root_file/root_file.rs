use crate::sys_utility::{
    addr::addr::BlockAddr,
    bitmap::block_bit_map::BlockBitmap,
    file::{zdir::ZDir, zfile::ZFile},
};
use std::{
    cell::RefCell,
    clone,
    sync::{Arc, Mutex},
};

use super::error::FileSystemOperationError;

#[derive(Debug)]
pub enum VFile {
    ZFile(ZFile),
    ZDir(ZDir),
}

struct RootFile {
    raw: Arc<Mutex<RawRootFile>>,
}
#[derive(Debug)]
pub struct RawRootFile {
    bitmap: BlockBitmap,
    dir: BlockAddr,
}

impl RawRootFile {
    const TEST_ADDR: BlockAddr = BlockAddr { addr: 775 };
    pub fn new() -> RawRootFile {
        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
                                                                           // let zd=ZDir::open(BlockAddr { addr: 253 }).unwrap();
        Self {
            bitmap: bit_map,
            dir: Self::TEST_ADDR,
        }
    }

    pub fn get_file(&mut self, path: &str) -> Result<VFile, FileSystemOperationError> {
        let pathv = Self::parse_path(path);
        let mut current_dir = ZDir::open(self.dir).unwrap();
        for i in 1..pathv.len() - 1 {
            let addr = current_dir
                .get_item_block_entry(pathv.get(i).unwrap())
                .unwrap();
            current_dir = match ZDir::open(addr) {
                Ok(x) => x,
                Err(_) => {
                    return Result::Err(FileSystemOperationError::NotFoundError(format!(
                        "we cannot found the file:{path}"
                    )))
                }
            }
        }
        let addr = current_dir
            .get_item_block_entry(pathv.get(pathv.len() - 1).unwrap())
            .unwrap();
        let ans = ZFile::open(addr);

        Ok(VFile::ZFile(ans))
    }

    pub fn parse_path(path: &str) -> Vec<String> {
        path.split('/').map(|x| x.to_string()).collect()
    }
}

// impl Drop for RawRootFile{
//     fn drop(&mut self) {
//         let mut dir=self.dir.take().unwrap();
//         dir.close();
//     }
// }

#[cfg(test)]
#[test]
fn test_parse() {
    println!("{:?}", RawRootFile::parse_path("/warth/gogo/rust"));
}

#[test]
fn raw_test() {
    let mut zd = ZDir::open(BlockAddr::new(775)).unwrap();
    zd.status();
    // zd.mkdir("warth1");
    zd.ls();

    let warth = zd.get_item_block_entry("warth").unwrap();

    let mut warth = ZDir::open(warth).unwrap();

    println!("{:?}", warth);
    // zd.mkdir("warth");
    // let warth=zd.get_item_block_entry("warth").unwrap();
    // let warth=ZDir::open(warth).unwrap();
    // println!("{:?}",warth);
}

#[test]
fn test_get_file() {
    let mut raw = RawRootFile::new();
    let a = raw.get_file("/warth/gogo").unwrap();
    println!("{:?}", a);
}
