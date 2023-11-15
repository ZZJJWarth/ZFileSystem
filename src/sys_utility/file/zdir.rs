use std::mem::transmute;

use crate::{
    file_shell::root_file::error::FileSystemOperationError,
    sys_utility::{
        addr::addr::BlockAddr,
        bitmap::block_bit_map::BlockBitmap,
        file::zfile::ZFile,
        super_block::{
            super_block::SuperBlock,
            unwarper::{get_bitmap, unwrap_bitmap},
        },
    },
};

use super::{
    dir_servant::{DirItem, DirServant},
    file_err::DirDeleleError,
    raw_f::{FileType, RawF},
    raw_file::RawFile,
};
#[derive(Debug)]
pub struct ZDir {
    pub servant: DirServant,
}

impl ZDir {
    ///无中生有地生成一个ZDir，它会安排好底层block中的所有东西
    pub fn new() -> Result<Self, FileSystemOperationError> {
        // let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        let mut bm = get_bitmap()?;
        let mut bit_map = unwrap_bitmap(&bm)?; //获取位图
        let b = bit_map.get_free_block().unwrap(); //从位图中获取空闲块
        drop(bit_map); //此时已经不需要位图了，必须要释放，否则会引发死锁
        let raw = RawFile::new(super::raw_f::FileType::Dir, b); //生成raw文件L
        let mut serve = DirServant::new(raw, 0); //生成servant
                                                 // drop(bit_map);
        serve.init();
        let mut zd = ZDir { servant: serve };
        zd.write_self();
        Ok(zd)
    }

    pub fn new_root(root_entry: BlockAddr) -> Self {
        let raw = RawFile::new(super::raw_f::FileType::Dir, root_entry);
        let mut serve = DirServant::new(raw, 0);
        serve.init();
        let mut zd = ZDir { servant: serve };
        zd.write_self();
        zd
    }

    pub fn init_raw(raw: RawFile) {
        let mut serve = DirServant::new(raw, 0);
        serve.init();
    }

    ///通过一个BlockAddr打开一个ZDir
    pub fn open(addr: BlockAddr) -> Result<Self, ()> {
        let mut f = RawFile::open(addr).unwrap();
        if (f.get_type() != super::raw_f::FileType::Dir) {
            println!("this is not a dir!");
            return Err(());
        }
        let mut buf: Vec<u8> = vec![];
        f.read(0, &mut buf, ZDirPack::PACK_SIZE).unwrap();
        // println!("{:?}",f);
        let mut data: [u8; ZDirPack::PACK_SIZE as usize] = [0; ZDirPack::PACK_SIZE as usize];
        data.copy_from_slice(&buf);
        let zd = ZDirPack::from_u8(data);
        let zd = zd.into_zdir();
        Ok(zd)
    }
    ///关闭一个Dir
    pub fn close(&mut self) -> Result<(), FileSystemOperationError> {
        self.write_self()?;
        self.servant.file().close();
        Ok(())
    }

    pub fn add_file(&mut self, name: &str) -> u32 {
        todo!()
    }

    fn add_item(&mut self, item: DirItem) -> Result<(), ()> {
        todo!()
    }
    ///很重要的一个函数，它会写入一个Dir的头部，把DirPack写入头部
    fn write_self(&mut self) -> Result<usize, FileSystemOperationError> {
        let a = ZDirPack::new(self);
        let a = ZDirPack::into_u8(a).to_vec();
        self.servant.file().write(0, &a, ZDirPack::PACK_SIZE)
    }

    pub fn get_item_num(&self) -> u32 {
        self.servant.get_item_num()
    }
    ///获取自己的入口
    pub fn get_block_entry(&self) -> BlockAddr {
        self.servant.get_block_entry()
    }
    ///列出当前文件夹的文件列表
    pub fn ls(&self) -> String {
        self.servant.command_ls()
    }
    ///输入文件名字和文件类型创建文件（下面是这个的语法糖，都是一样的功能）
    pub fn insert_item(
        &mut self,
        name: &str,
        file_type: FileType,
        owner_u_id: u8,
    ) -> Result<(), FileSystemOperationError> {
        self.servant.new_dir_item(name, file_type, owner_u_id)
    }
    ///获得一个文件的入口
    pub fn get_item_block_entry(&mut self, name: &str) -> Option<BlockAddr> {
        self.servant.find_item(name)
    }
    ///删除一个文件（输入名字删除）
    pub fn del_item(&mut self, name: &str) -> Result<(), FileSystemOperationError> {
        self.servant.del_item(name)
    }
    ///查看目录块的状态
    pub fn status(&mut self) {
        self.servant.item_status();
        println!();
    }
    ///在当前目录下，创建一个文件
    pub fn touch(&mut self, name: &str, owner_u_id: u8) -> Result<(), FileSystemOperationError> {
        self.servant.new_dir_item(name, FileType::File, owner_u_id)
    }
    ///在当前目录下，创建一个目录
    pub fn mkdir(&mut self, name: &str, owner_u_id: u8) -> Result<(), FileSystemOperationError> {
        self.servant.new_dir_item(name, FileType::Dir, owner_u_id)
    }
    ///在当前目录下，按照文件名获得某个文件的u_id
    pub fn get_owner_id(&self, name: &str) -> Result<u8, FileSystemOperationError> {
        self.servant.get_owner_id(name)
    }
    ///host copy
    pub fn host_cp(
        &mut self,
        source_path: &str,
        file_name: &str,
        owner_u_id: u8,
    ) -> Result<(), FileSystemOperationError> {
        self.servant.host_cp(source_path, file_name, owner_u_id)
    }

    pub fn dir_ls_l(&self)->Result<String,FileSystemOperationError>{
        self.servant.dir_ls_l()
    }
}

impl Drop for ZDir {
    fn drop(&mut self) {
        self.close();
    }
}

///本结构体是为了实现ZDir的转换，
///经过转换后，ZDir变成了ZDirPack，ZDirPack会把自己转换成u8数组，以便写入
pub struct ZDirPack {
    num: u32,
    entry: BlockAddr,
}

impl ZDirPack {
    ///本常量表示ZDirPack的size
    pub const PACK_SIZE: u32 = 8;

    ///输入一个ZDir，输出一个ZDirPack，ZDirPack有利于转换u8数组
    fn new(zd: &ZDir) -> Self {
        let n = zd.get_item_num();
        let entry = zd.get_block_entry();
        Self { num: n, entry }
    }
    ///消耗ZDirPack，生成ZDir
    fn into_zdir(self) -> ZDir {
        let rf = RawFile::open(self.entry).unwrap(); //不能使用new函数！
        let num = self.num;
        let serve = DirServant::new(rf, num);
        ZDir { servant: serve }
    }
    ///从u8数组转换成ZDirPack
    fn from_u8(data: [u8; Self::PACK_SIZE as usize]) -> Self {
        unsafe { transmute::<[u8; Self::PACK_SIZE as usize], Self>(data) }
    }
    ///从ZDirPack转换成u8数组
    fn into_u8(data: Self) -> [u8; Self::PACK_SIZE as usize] {
        unsafe { transmute::<Self, [u8; Self::PACK_SIZE as usize]>(data) }
    }
}

#[cfg(test)]
// #[test]
fn test_new() {
    let mut zd = ZDir::new().unwrap();
    println!("{:?}", zd);
    zd.close();
}

#[test]
fn test_open() {
    let mut zd = ZDir::open(BlockAddr { addr: 254 }).unwrap();
    // zd.mkdir("name");
    println!("{:?}", zd);
    // zd.close();
}

#[test]
fn test_see() {
    let mut rf = RawFile::open(BlockAddr { addr: 83 }).unwrap();
    let mut buf: Vec<u8> = vec![];
    rf.read(0, &mut buf, ZDirPack::PACK_SIZE);
    println!("{:?}", buf);
}

#[test]
fn test_dir_item() {
    let mut zd = ZDir::open(BlockAddr { addr: 115 }).unwrap();
    // zd.insert_item("hello China1", BlockAddr { addr: 0 }).unwrap();
    // zd.insert_item("hello China2", BlockAddr { addr: 0 }).unwrap();
    // zd.insert_item("hello China3", BlockAddr { addr: 0 }).unwrap();
    // zd.insert_item("hello China4", BlockAddr { addr: 0 }).unwrap();
    // zd.insert_item("hello China5", BlockAddr { addr: 0 }).unwrap();
    // zd.insert_item("hello China6", BlockAddr { addr: 0 }).unwrap();
    // zd.insert_item("hello China7", BlockAddr { addr: 0 }).unwrap();
    // zd.insert_item("hello China8", BlockAddr { addr: 0 }).unwrap();
    // zd.insert_item("hello China9", BlockAddr { addr: 0 }).unwrap();
    // zd.insert_item(
    //     "this is a long long long long long long long long long long item!",
    //     BlockAddr { addr: 0 },
    // )
    // .unwrap();
    zd.ls();
    zd.close();
}

#[test]
fn test_item() {
    // let mut zd = ZDir::open(BlockAddr::new(212)).unwrap();
    // // println!("{:?}",zd);
    // // zd.insert_item("File1", FileType::File);
    // zd.ls();
    // println!("\nDir1's entry is {:?}", zd.get_item_block_entry("File1"));
    // let addr = zd.get_item_block_entry("File1").unwrap();
    // let mut zd1 = ZFile::open(addr);
    // // zd1.char_write(0, 1, vec!['h']);
    // println!("{:?}", zd1.char_read(0, 1));
    // zd1.close();
    // // println!("hello's entry is :{:?}",zd.get_item_block_entry("File6"));
    // // println!("{:?}",zd);
    // zd.close();
}

#[test]
fn status() {
    let mut zd = ZDir::open(BlockAddr::new(245)).unwrap();
    // zd.mkdir("dir3");
    // zd.touch("file2");
    println!("{:?}", zd.servant.dir_empty());
    let entry = zd.get_item_block_entry("dir3").unwrap();
    let mut dir3 = ZDir::open(entry).unwrap();
    // dir3.mkdir("dir4");
    dir3.status();
    dir3.ls();
    dir3.close();
    zd.del_item("dir3");
    // zd.ls();
    // zd.status();
    zd.close();
}

#[test]
fn dead() {
    let mut sb = SuperBlock::init_main("../test");
    // let zd=ZDir::new();
    let mut bm = get_bitmap().unwrap();
    let mut bit_map = unwrap_bitmap(&bm).unwrap();
    let b = bit_map.get_free_block().unwrap();
    drop(bit_map);
    let raw = RawFile::new(super::raw_f::FileType::Dir, b); //生成raw文件L
    let mut serve = DirServant::new(raw, 0); //生成servant
                                             // drop(bit_map);
                                             // serve.init();
}
