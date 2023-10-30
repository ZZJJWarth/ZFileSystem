use std::mem::transmute;

use crate::sys_utility::{
    addr::addr::BlockAddr, bitmap::block_bit_map::BlockBitmap, file::zfile::ZFile,
};

use super::{
    dir_servant::{DirItem, DirServant},
    raw_f::{FileType, RawF},
    raw_file::RawFile,
};
#[derive(Debug)]
pub struct ZDir {
    pub servant: DirServant,
}

impl ZDir {
    ///无中生有地生成一个ZDir，它会安排好底层block中的所有东西
    pub fn new() -> Self {
        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        let b = bit_map.get_free_block().unwrap();
        let raw = RawFile::new(super::raw_f::FileType::Dir, b);
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
    pub fn close(mut self) {
        self.write_self();
        self.servant.file().close();
    }

    pub fn add_file(&mut self, name: &str) -> u32 {
        todo!()
    }

    fn add_item(&mut self, item: DirItem) -> Result<(), ()> {
        todo!()
    }
    ///很重要的一个函数，它会写入一个Dir的头部，把DirPack写入头部
    fn write_self(&mut self) -> Result<(), ()> {
        let a = ZDirPack::new(self);
        let a = ZDirPack::into_u8(a).to_vec();
        self.servant.file().write(0, &a, ZDirPack::PACK_SIZE)
    }

    pub fn get_item_num(&self) -> u32 {
        self.servant.get_item_num()
    }

    pub fn get_block_entry(&self) -> BlockAddr {
        self.servant.get_block_entry()
    }

    pub fn ls(&mut self) {
        self.servant.command_ls();
    }

    pub fn insert_item(&mut self, name: &str, file_type: FileType) -> Result<(), ()> {
        self.servant.new_dir_item(name, file_type)
    }

    pub fn get_item_block_entry(&mut self, name: &str) -> Option<BlockAddr> {
        self.servant.find_item(name)
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
#[test]
fn test_new() {
    let zd = ZDir::new();
    println!("{:?}", zd);
    zd.close();
}

#[test]
fn test_open() {
    let zd = ZDir::open(BlockAddr { addr: 211 }).unwrap();
    println!("{:?}", zd);
    zd.close();
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
    let mut zd = ZDir::open(BlockAddr::new(212)).unwrap();
    // println!("{:?}",zd);
    // zd.insert_item("File1", FileType::File);
    zd.ls();
    println!("\nDir1's entry is {:?}", zd.get_item_block_entry("File1"));
    let addr = zd.get_item_block_entry("File1").unwrap();
    let mut zd1 = ZFile::open(addr);
    // zd1.char_write(0, 1, vec!['h']);
    println!("{:?}", zd1.char_read(0, 1));
    zd1.close();
    // println!("hello's entry is :{:?}",zd.get_item_block_entry("File6"));
    // println!("{:?}",zd);
    zd.close();
}
