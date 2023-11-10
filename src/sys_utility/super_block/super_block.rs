use std::{
    fs::File,
    sync::{Arc, Mutex},
};

use crate::{
    file_shell::{
        file_table::file_table::FileTable,
        root_file::{error::FileSystemOperationError, root_file::RawRootFile},
    },
    sys_utility::{
        addr::addr::BlockAddr, bitmap::block_bit_map::BlockBitmap, config::config::BLOCK_SIZE,
    },
    SUPER_BLOCK,
};

#[derive(Debug)]
pub struct SuperBlock {
    block_num: u32,           //磁盘总块数
    reserve_num: u32,         //保留给超级块的块数
    bitmap_num: u32,          //保留给bitmap的块数
    root_dir_addr: BlockAddr, //根目录的地址
    bitmap: Option<Arc<Mutex<BlockBitmap>>>,
    file_table: Option<Arc<Mutex<FileTable>>>,
}

impl SuperBlock {
    pub fn new(disk_path: &str) -> Result<SuperBlock, FileSystemOperationError> {
        let f = File::open(disk_path);
        let f = match f {
            Ok(s) => s,
            Err(_) => return Err(FileSystemOperationError::DiskError(format!("磁盘打开错误"))),
        };
        let disk_len = f.metadata().unwrap().len() as u32;
        let block_num = disk_len / BLOCK_SIZE;
        let reserve_num: u32 = 1;
        let bitmap_num = block_num / (BLOCK_SIZE / 4);
        let root_dir_addr = BlockAddr::new(reserve_num + bitmap_num + 1);
        Ok(SuperBlock {
            block_num,
            reserve_num,
            bitmap_num,
            root_dir_addr,
            bitmap: None,
            file_table: None,
        })
    }

    pub fn init(&mut self) -> Result<(), FileSystemOperationError> {
        let bitmap = BlockBitmap::new(
            BlockAddr::new(self.reserve_num),
            self.bitmap_num,
            self.reserve_num + self.reserve_num + self.bitmap_num,
        );
        let bitmap = Arc::new(Mutex::new(bitmap));
        let root_file = RawRootFile::new(Arc::clone(&bitmap), self.root_dir_addr);
        let file_table = FileTable::init_new(root_file);
        self.bitmap = Some(bitmap);
        self.file_table = Some(Arc::new(Mutex::new(file_table)));
        Ok(())
    }

    pub fn get_file_table(&self) -> Option<Arc<Mutex<FileTable>>> {
        self.file_table.clone()
    }

    pub fn get_bitmap(&self) -> Option<Arc<Mutex<BlockBitmap>>> {
        self.bitmap.clone()
    }

    pub fn init_main(path: &str) -> Result<(), FileSystemOperationError> {
        let mut sb = SuperBlock::new(path)?;
        unsafe {
            SUPER_BLOCK = Some(Arc::new(Mutex::new(sb)));
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_new() {
    let sb = SuperBlock::new("../test");
    println!("{:?}", sb);
}
