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
        addr::addr::BlockAddr,
        bitmap::block_bit_map::BlockBitmap,
        config::config::{BLOCK_SIZE, END_NUM, FILE_PATH},
        file::zdir::ZDir,
    },
    SUPER_BLOCK,
};

use super::{
    super_pack::SuperPack,
    unwarper::{get_bitmap, unwrap_bitmap},
};

#[derive(Debug)]
pub struct SuperBlock {
    block_num: u32,           //磁盘总块数
    reserve_num: u32,         //保留给超级块的块数
    bitmap_num: u32,          //保留给bitmap的块数
    root_dir_addr: BlockAddr, //根目录的地址
    magic_num: u64,           //魔数
    first_init: bool,         //是否已经初始化
    user_manager_entry: u32,  //用户管理器的地址
    bitmap: Option<Arc<Mutex<BlockBitmap>>>,    //位图临界资源
    file_table: Option<Arc<Mutex<FileTable>>>,  //文件哈希表临界资源
}

impl SuperBlock {
    pub const MAGIC_FLAG: u64 = 1130423011300520113;
    pub const SUPER_BLOCK_SIZE: u32 = 32;
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
            first_init: true,
            magic_num: Self::MAGIC_FLAG,
            user_manager_entry: 0,
            bitmap: None,
            file_table: None,
        })
    }

    pub fn init(&mut self) -> Result<(), FileSystemOperationError> {
        // let now_block=SuperPack::load();
        let mut bitmap = BlockBitmap::new(
            BlockAddr::new(self.reserve_num),
            self.bitmap_num,
            self.reserve_num + self.reserve_num + self.bitmap_num,
        );
        if bitmap.check_block_empty(self.root_dir_addr) {
            bitmap.set_value(self.root_dir_addr, END_NUM);
        }
        // bitmap.set_value(BlockAddr { addr:  }, value)

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
        let temp = SuperPack::load(path)?;
        let mut sb = if temp.is_legal() {
            SuperBlock::from(temp)
        } else {
            SuperBlock::new(path)?
        };
        // let mut sb = SuperBlock::new(path)?;
        sb.init();
        // sb.write_super_block(path)?;

        // sb.write_super_block(path)?;
        unsafe {
            SUPER_BLOCK = Some(Arc::new(Mutex::new(sb)));
        }

        Ok(())
    }

    pub fn init_rootdir() {
        let a = unsafe { &SUPER_BLOCK };
        let a = match a {
            Some(s) => s,
            None => {
                return;
            }
        };
        let mut flag = false;
        let mut dir_root: Option<BlockAddr> = None;
        {
            let mut temp = a.lock().unwrap();
            flag = temp.first_init;
            dir_root = Some(temp.root_dir_addr);
        }
        if flag {
            ZDir::new_root(dir_root.unwrap());
            let mut bm = get_bitmap().unwrap();
            let mut bit_map = unwrap_bitmap(&bm).unwrap();
            bit_map.init();
            drop(bit_map);
            let mut temp = a.lock().unwrap();

            temp.first_init = false;
            temp.write_super_block(FILE_PATH);
        }
        //处理死锁
    }

    fn into_super_pack(&self) -> SuperPack {
        SuperPack {
            block_num: self.block_num,
            reserve_num: self.reserve_num,
            bitmap_num: self.bitmap_num,
            root_dir_addr: self.root_dir_addr,
            magic_num: self.magic_num,
            first_init: self.first_init,
        }
    }

    fn write_super_block(&self, path: &str) -> Result<(), FileSystemOperationError> {
        let pk = self.into_super_pack();
        pk.dump(path)
    }
}

impl From<SuperPack> for SuperBlock {
    fn from(value: SuperPack) -> Self {
        Self {
            block_num: value.block_num,
            reserve_num: value.reserve_num,
            bitmap_num: value.bitmap_num,
            root_dir_addr: value.root_dir_addr,
            magic_num: value.magic_num,
            first_init: value.first_init,
            user_manager_entry: 0,
            bitmap: None,
            file_table: None,
        }
    }
}

#[cfg(test)]
#[test]
fn test_new() {
    use crate::{
        file_shell::bin::helper::{ft_unwrap, get_ft},
        sys_utility::{file::zfile::ZFile, super_block::unwarper::unwrap_bitmap},
    };

    use super::unwarper::get_bitmap;

    let mut sb = SuperBlock::init_main("../test3");

    let ft = get_ft().unwrap();
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft).unwrap();

    let a = ft.open("test").unwrap();
    drop(ft);
    let b = a.write().unwrap();
    // b.file_cp("/dir1", "hihi");
    let ft = get_ft().unwrap();
    let ft = ft.lock();
    let mut ft = ft_unwrap(ft).unwrap();
    ft.check();
    // SuperBlock::init_rootdir();
    // unsafe{println!("{:?}",&SUPER_BLOCK);}
    // // let zd=ZDir::new();
    // let mut bm=get_bitmap().unwrap();
    // let mut bit_map=unwrap_bitmap(&bm).unwrap();
    // let a=bit_map.get_free_block().unwrap();

    // println!("{:?}",a);

    // let mut zd=ZDir::open(BlockAddr { addr: 2 }).unwrap();
    // let mut zf=ZFile::open(BlockAddr { addr: 4 });
    // zf.write(format!("123456"));
    // println!("{:?}",zf);
    // zf.close();
    // zd.touch("hh");
    // SuperBlock::init_rootdir();
}
