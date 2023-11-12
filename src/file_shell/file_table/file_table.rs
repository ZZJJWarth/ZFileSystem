use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::file_shell::root_file::{
    self,
    error::FileSystemOperationError,
    root_file::{RawRootFile, VFile},
};
#[derive(Debug)]
pub struct FileTable {
    hash_map: HashMap<String, Arc<RwLock<VFile>>>,
    root_file: RawRootFile,
}

impl FileTable {
    ///创建新的FileTable
    // pub fn new() -> FileTable {
    //     let hash_map: HashMap<String, Arc<RwLock<VFile>>> = HashMap::new();
    //     let root_file = RawRootFile::new();
    //     FileTable {
    //         hash_map,
    //         root_file,
    //     }
    // }

    pub fn init_new(root_file: RawRootFile) -> FileTable {
        let hash_map: HashMap<String, Arc<RwLock<VFile>>> = HashMap::new();
        FileTable {
            hash_map,
            root_file,
        }
    }
    ///供用户使用的open函数，根据地址获得文件的指针,这是打开文件
    ///本函数接收的是绝对地址
    pub fn open(&mut self, addr: &str) -> Result<Arc<RwLock<VFile>>, FileSystemOperationError> {
        let addr = String::from(addr);
        // for i in self.hash_map.keys() {
        //     if addr == *i {
        //         return self.hash_map.get(i).unwrap().clone();
        //     }
        // }
        match self.hash_map.get(&addr) {
            Some(f) => return Ok(f.clone()),
            None => {}
        };
        let f = self.root_file.get_raw(&addr)?;
        let f = Arc::new(RwLock::new(f));
        self.hash_map.insert(addr, Arc::clone(&f));
        return Ok(f);
    }

    ///关闭某个文件
    fn close(addr: &str) {
        todo!()
    }

    pub fn check(&mut self) -> Result<String, FileSystemOperationError> {
        let mut count = 0;
        let mut err = 0;
        for i in self.hash_map.values() {
            let mut guard = i.write().unwrap();
            // println!("{:?}",i);
            match guard.close() {
                Ok(_) => {
                    count += 1;
                }
                Err(_) => {
                    err += 1;
                }
            }
        }
        Ok(format!(
            "完成一致性检测，共{}个文件，{}个文件完成检测，{}个文件检测出现错误",
            count + err,
            count,
            err
        ))
    }
}

#[cfg(test)]
#[test]
fn test_open_raw() {
    use crate::{
        file_shell::bin::{
            self,
            helper::{ft_unwrap, get_ft},
        },
        sys_utility::{
            file::zfile::ZFile,
            super_block::{
                super_block::SuperBlock,
                unwarper::{self, unwrap_bitmap},
            },
        },
    };

    use unwarper::get_bitmap;

    let mut sb = SuperBlock::init_main("../test3");
    SuperBlock::init_rootdir();
    let ft = get_ft().unwrap();
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft).unwrap();

    ft.open("test");
    // println!("{:?}",ft);
    println!("{:?}", ft.check());
}
