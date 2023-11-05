use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::file_shell::root_file::root_file::{RawRootFile, VFile};

pub struct FileTable {
    hash_map: HashMap<String, Arc<RwLock<VFile>>>,
    root_file: RawRootFile,
}

impl FileTable {
    ///创建新的FileTable
    fn new() {
        todo!()
    }
    ///供用户使用的open函数，根据地址获得文件的指针
    fn open(&mut self, addr: &str) -> Arc<RwLock<VFile>> {
        let addr = String::from(addr);
        for i in self.hash_map.keys() {
            if addr == *i {
                return self.hash_map.get(i).unwrap().clone();
            }
        }
        let f = self.root_file.get_file(&addr).unwrap();
        let f = Arc::new(RwLock::new(f));
        self.hash_map.insert(addr, Arc::clone(&f));
        return f;
    }
    ///关闭某个文件
    fn close(addr: &str) {
        todo!()
    }
}
