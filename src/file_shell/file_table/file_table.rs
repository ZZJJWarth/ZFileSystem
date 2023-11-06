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
    pub fn new() ->FileTable{
        let hash_map:HashMap<String, Arc<RwLock<VFile>>>=HashMap::new();
        let root_file=RawRootFile::new();
        FileTable{hash_map,root_file}
    }
    ///供用户使用的open函数，根据地址获得文件的指针
    pub fn open(&mut self, addr: &str) -> Arc<RwLock<VFile>> {
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


#[cfg(test)]
#[test]
fn test_open_raw(){
    use std::ops::Deref;

    let mut ft=FileTable::new();
    let a=ft.open("/warth");
    let a=a.as_ref().read().unwrap().deref();
}