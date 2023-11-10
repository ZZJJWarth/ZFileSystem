use crate::sys_utility::{
    addr::addr::BlockAddr,
    bitmap::block_bit_map::BlockBitmap,
    config::config::NON_OCCUPY_NUM,
    file::{
        raw_f::{FileType, RawF},
        raw_file::RawFile,
        zdir::ZDir,
        zfile::ZFile,
    },
};
use std::{
    cell::RefCell,
    clone,
    fmt::format,
    sync::{Arc, Mutex},
};

use super::error::FileSystemOperationError;

#[derive(Debug)]
pub enum VFile {
    ZFile(ZFile),
    ZDir(ZDir),
}

impl VFile {
    pub fn dir_mkdir(&mut self, name: &str) -> Result<(), FileSystemOperationError> {
        match self {
            VFile::ZFile(_) => Err(FileSystemOperationError::NotDirError(format!(
                "mkdir:这里需要一个目录，但是这里却是文件"
            ))),
            VFile::ZDir(zdir) => zdir.mkdir(name),
        }
    }

    pub fn dir_touch(&mut self, name: &str) -> Result<(), FileSystemOperationError> {
        match self {
            VFile::ZFile(_) => Err(FileSystemOperationError::NotDirError(format!(
                "touch:这里需要一个目录，但是这里却是文件"
            ))),
            VFile::ZDir(zdir) => zdir.touch(name),
        }
    }

    pub fn file_cat(&self) -> Result<String, FileSystemOperationError> {
        // todo!()
        match self {
            VFile::ZDir(_) => Err(FileSystemOperationError::NotFileError(format!(
                "cat:这里需要一个文件，但是这里却是目录"
            ))),
            VFile::ZFile(f) => Ok(f.cat()),
        }
    }

    pub fn file_write(&mut self, content: String) -> Result<usize, FileSystemOperationError> {
        match self {
            VFile::ZDir(_) => Err(FileSystemOperationError::NotFileError(format!(
                "cat:这里需要一个文件，但是这里却是目录"
            ))),
            VFile::ZFile(f) => f.write(content),
        }
    }
}

struct RootFile {
    raw: Arc<Mutex<RawRootFile>>,
}
#[derive(Debug)]
pub struct RawRootFile {
    bitmap: Arc<Mutex<BlockBitmap>>,
    dir: BlockAddr,
}

impl RawRootFile {
    const TEST_ADDR: BlockAddr = BlockAddr { addr: 775 };
    // pub fn new() -> RawRootFile {
    //     let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
    //                                                                        // let zd=ZDir::open(BlockAddr { addr: 253 }).unwrap();
    //     Self {
    //         bitmap: bit_map,
    //         dir: Self::TEST_ADDR,
    //     }
    // }

    pub fn new(bitmap: Arc<Mutex<BlockBitmap>>, dir: BlockAddr) -> RawRootFile {
        Self { bitmap, dir }
    }

    fn get_addr(&mut self, path: &str) -> Result<BlockAddr, FileSystemOperationError> {
        let pathv = Self::parse_path(path);
        // println!("{:?}", pathv);
        if pathv.len() == 1 {
            return Ok(self.dir);
        }
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
        let addr = current_dir.get_item_block_entry(pathv.get(pathv.len() - 1).unwrap());
        let addr = match addr {
            Some(b) => b,
            None => {
                return Result::Err(FileSystemOperationError::NotFoundError(format!(
                    "we cannot found the file:{path}"
                )))
            }
        };
        return Ok(addr);
    }

    pub fn get_raw(&mut self, path: &str) -> Result<VFile, FileSystemOperationError> {
        let addr = match self.get_addr(path) {
            Ok(addr) => addr,
            Err(e) => return Err(e),
        };
        let ans = RawFile::open(addr).unwrap();
        match ans.get_type() {
            FileType::File => {
                let f = ZFile::open(addr);
                return Ok(VFile::ZFile(f));
            }
            FileType::Dir => {
                let d = ZDir::open(addr).unwrap();
                return Ok(VFile::ZDir(d));
            }
        }
    }

    pub fn get_file(&mut self, path: &str) -> Result<VFile, FileSystemOperationError> {
        let addr = match self.get_addr(path) {
            Ok(addr) => addr,
            Err(e) => return Err(e),
        };
        let ans = ZFile::open(addr);

        Ok(VFile::ZFile(ans))
    }

    pub fn get_dir(&mut self, path: &str) -> Result<VFile, FileSystemOperationError> {
        let addr = match self.get_addr(path) {
            Ok(addr) => addr,
            Err(e) => return Err(e),
        };
        let ans = ZDir::open(addr).unwrap();

        Ok(VFile::ZDir(ans))
    }

    pub fn parse_path(path: &str) -> Vec<String> {
        let mut ans: Vec<String> = path.split('/').map(|x| x.to_string()).collect();
        if ans.get(ans.len() - 1).unwrap() == "" {
            ans.pop();
            ans
        } else {
            ans
        }
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
    println!("{:?}", RawRootFile::parse_path("/warth/gogo"));
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

// #[test]
// fn test_get_file() {
//     let mut raw = RawRootFile::new();
// let a = raw.get_file("/warth/gogo");
//     println!("{:?}", a);
// }
