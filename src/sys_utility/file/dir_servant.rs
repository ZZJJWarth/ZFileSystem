use std::{
    collections::btree_map::Keys,
    fs::File,
    mem::transmute,
    ops::{Add, AddAssign, Sub},
    process::Output,
    vec,
};

use crate::{
    file_shell::root_file::error::FileSystemOperationError,
    sys_utility::{
        addr::addr::BlockAddr, bitmap::block_bit_map::BlockBitmap, block::block_servant::DataPack,
    },
};

use super::{
    file_err::DirDeleleError,
    raw_file::RawFile,
    zdir::{ZDir, ZDirPack},
    zfile::ZFile,
};

use super::raw_f::{FileType, RawF};

///帮助zdir创建、删除、查找、遍历目录项
#[derive(Debug)]
pub struct DirServant {
    raw: RawFile,
    num: u32,
}

impl DirServant {
    const DIR_BLOCK_NUM: usize = 32;
    const DIR_INIT_BLOCK_NUM: usize = 31;
    const HEAD_RESERVE_SIZE: usize = 32;
    pub fn init(&mut self) {
        let zeros = [0; Self::DIR_INIT_BLOCK_NUM * DirRawItem::ITEM_SIZE].to_vec();
        self.raw.add_write(&zeros, 32);
        self.raw.write(
            ZDirPack::PACK_SIZE + 8,
            &zeros,
            (Self::DIR_INIT_BLOCK_NUM * DirRawItem::ITEM_SIZE) as u32,
        );
        self.num = Self::DIR_INIT_BLOCK_NUM as u32;
    }

    pub fn extent_block_size(&mut self) {
        let zeros = [0; Self::DIR_BLOCK_NUM * DirRawItem::ITEM_SIZE].to_vec();
        self.raw
            .add_write(&zeros, (Self::DIR_BLOCK_NUM * DirRawItem::ITEM_SIZE) as u32);
        self.num = self.num + Self::DIR_BLOCK_NUM as u32;
    }

    pub fn new(raw: RawFile, num: u32) -> Self {
        Self { raw, num }
    }

    pub fn get_block_entry(&self) -> BlockAddr {
        self.raw.get_block_entry()
    }

    pub fn get_item_num(&self) -> u32 {
        self.num
    }

    pub fn file(&mut self) -> &mut RawFile {
        &mut self.raw
    }

    ///傻瓜式地插入一个Item
    fn insert_item(
        &mut self,
        item: DirRawItem,
        addr: ItemAddr,
    ) -> Result<usize, FileSystemOperationError> {
        if (addr.get_addr() >= self.num) {
            println!(
                "DirServent 尝试越界写入，请尝试增加容量再写入\n写入坐标：{:?}\n当前空间：{:?}",
                addr, self.num
            );
            panic!();
        }

        let mut buf = item.into_u8().to_vec();
        // println!("{:?}",buf.len());
        self.raw
            .write(addr.get_offset(), &mut buf, DirRawItem::ITEM_SIZE as u32)
    }
    ///傻瓜式地读取一个Item
    fn get_item(&self, addr: ItemAddr) -> Result<DirRawItem, ()> {
        if (addr.get_addr() >= self.num) {
            println!("DirServent 尝试越界读取，请尝试增加容量写入后再读取\n读取坐标：{:?}\n当前空间：{:?}",addr,self.num);
            panic!();
        }
        let mut buf: Vec<u8> = vec![];

        self.raw
            .read(addr.get_offset(), &mut buf, DirRawItem::ITEM_SIZE as u32)?;
        let mut temp: [u8; DirRawItem::ITEM_SIZE as usize] = [0; DirRawItem::ITEM_SIZE as usize];
        for i in 0..DirRawItem::ITEM_SIZE as usize {
            temp[i] = buf[i];
        }
        Ok(DirRawItem::from_u8(temp))
    }
    ///输入一个文件名，向文件夹中插入一个diritem,并产生相应的文件
    pub fn new_dir_item(
        &mut self,
        name: &str,
        file_type: FileType,
    ) -> Result<(), FileSystemOperationError> {
        let check = self.has_name(name);
        match check {
            Some(_) => {
                return Err(FileSystemOperationError::ExistNameError(format!(
                    "Already has a file/dir named:{}",
                    name
                )));
            }
            None => {}
        };
        // let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        // let entry=bit_map.get_free_block().unwrap();

        // let raw=RawFile::new(file_type,entry);
        let mut entry: BlockAddr;
        match file_type {
            FileType::Dir => {
                let mut temp = ZDir::new();
                entry = temp.get_block_entry();
                temp.close();
            }
            FileType::File => {
                let mut temp = ZFile::new();
                entry = temp.get_block_entry();
                temp.close();
            }
        }
        let generator = DirItemGenerateIter::new(name, entry, file_type);
        // println!("{:?}",generator);

        let mut item_addr = self.find_emtpy_gap(generator.len() as u32);

        // println!("{:?}",item_addr);
        for i in generator {
            // println!("{:?}",i);
            let item = DirRawItem::new(i);
            self.insert_item(item, item_addr);
            // let a=self.get_item(item_addr).unwrap();
            // println!("{:?}",a);
            item_addr.step();
        }
        Ok(())
    }

    pub fn command_ls(&self) -> String {
        let mut ans: Vec<String> = vec![];
        let range = ItemAddrRange::new(ItemAddr::new(0), ItemAddr::new(self.num));
        for i in range.iter() {
            // println!("{:?}",i);
            let temp = self.get_item(i).unwrap().into_dir_item();
            // println!("{:?}",temp);
            match temp {
                DirItem::Short(item) => {
                    //todo:这里需要完成一个function，使得我可以输入ShortDirItem和它的Addr就可以获取全部文件名 done!
                    // println!("{:?}",temp);
                    let str = self.get_name(i);
                    let file_type = item.get_type();
                    match file_type {
                        FileType::Dir => {
                            ans.push(format!("<{}>\t", str));
                        }
                        FileType::File => {
                            ans.push(format!("{}\t", str));
                        }
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        ans.concat()
    }

    ///输入一个Itemaddr，获取一个Item的名字
    pub fn get_name(&self, addr: ItemAddr) -> String {
        let list = self.get_name_raw(addr);
        let a = list.iter().map(|x| *x as char).collect::<Vec<_>>();
        a.iter().collect()
    }

    fn get_name_raw(&self, addr: ItemAddr) -> Vec<u8> {
        let mut current = self.get_item(addr).unwrap().into_dir_item();
        let mut addr = addr;
        let mut ans: Vec<Vec<u8>> = vec![];
        let mut count = 0;
        let mut short_flag = false;

        loop {
            count += 1;
            if count > 100 {
                panic!("死循环了可能");
            }

            match self.get_flag(addr) {
                DirRawItem::LONG_END_FLAG => {
                    let temp = current.get_name().unwrap();
                    ans.push(temp);
                    break;
                }
                DirRawItem::NON_USE_FLAG => {
                    break;
                }
                DirRawItem::SHORT_FLAG => {
                    if short_flag {
                        break;
                    } else {
                        short_flag = true;
                        let temp = current.get_name().unwrap();
                        ans.push(temp);
                        addr.step();
                        current = self.get_item(addr).unwrap().into_dir_item();
                    }
                }
                _ => {
                    let temp = current.get_name().unwrap();
                    ans.push(temp);
                    addr.step();
                    current = self.get_item(addr).unwrap().into_dir_item();
                }
            }
        }

        ans.concat()
    }

    ///获取一个item的flag
    fn get_flag(&self, addr: ItemAddr) -> u8 {
        let item = self.get_item(addr).unwrap().into_dir_item();

        item.get_flag().unwrap()
    }

    ///根据所需要的长度，找到一块连续的空间，并返回首地址
    fn find_emtpy_gap(&mut self, length: u32) -> ItemAddr {
        let range = ItemAddrRange::new(ItemAddr::new(0), ItemAddr::new(self.num));
        let mut count = 0;
        let mut ans = ItemAddr::new(0);
        let mut flag = true;
        for i in range.iter() {
            let temp = self.get_item(i).unwrap().into_dir_item();
            if flag {
                ans = i;
                flag = false;
            }
            match temp {
                DirItem::None => {
                    count += 1;
                    if (count >= length) {
                        return ans;
                    }
                }
                _ => {
                    count = 0;
                    flag = true;
                }
            }
        }
        let temp = self.num;
        ///这里为了开发方便就限制了文件名的最大长度了，最大长度为1024个字节，如果需要扩展，则需要进行逻辑上的判断
        self.extent_block_size();
        ItemAddr { addr: temp }
    }

    ///通过一个name来查找是否有这个item
    fn has_name(&mut self, name: &str) -> Option<ItemAddr> {
        let range = ItemAddrRange::new(ItemAddr::new(0), ItemAddr::new(self.num));
        for i in range.iter() {
            let n = self.get_name(i);
            if (name.to_string() == n) {
                return Some(i);
            }
        }
        return None;
    }

    ///在目录项中查找是否有这个Item
    pub fn find_item(&mut self, name: &str) -> Option<BlockAddr> {
        let find = self.has_name(name);
        // println!("{:?}",self.get_item(ItemAddr { addr: 0 }));
        match find {
            Some(i) => Some(self.get_item_block_entry(i)),
            None => {
                println!("such a name is not found in dir:{}", name);
                None
            }
        }
    }

    fn get_item_block_entry(&mut self, addr: ItemAddr) -> BlockAddr {
        let item = self.get_item(addr).unwrap().into_dir_item();
        match item {
            DirItem::Short(s) => s.get_block(),
            _ => {
                println!(
                    "尝试在非ShortDirItem中读取Blockentry，尝试的Item为{:?},当前dir为{:?}",
                    item, self
                );
                panic!()
            }
        }
    }

    pub fn del_item(&mut self, name: &str) -> Result<(), DirDeleleError> {
        let find = self.has_name(name);
        let addr = match find {
            Some(addr) => addr,
            None => {
                return Err(DirDeleleError::NotFoundError(format!(
                    "not found {} in dir",
                    name
                )))
            }
        };
        let item = self.get_item(addr).unwrap().into_dir_item();
        let (file_type, entry) = match item {
            DirItem::Short(s) => (s.get_type(), s.get_block()),
            _ => {
                return Err(DirDeleleError::NotShortItemError(format!(
                    "找到的item不是shortitem，考虑get_item是否有问题"
                )))
            }
        };
        match file_type {
            FileType::Dir => {
                let mut zd = ZDir::open(entry).unwrap();
                if zd.servant.dir_empty() {
                    //空文件夹可以删除
                    zd.servant.raw.del();
                } else {
                    return Err(DirDeleleError::NotEmptyDirError(format!(
                        "删除失败，文件夹不为空"
                    )));
                }
            } //TODO：这里还有dir的问题要解决！
            FileType::File => {
                let mut f = ZFile::open(entry);
                f.del();
            }
        }
        self.set_empty_item(addr);
        Ok(())
    }

    fn set_empty_item(&mut self, addr: ItemAddr) {
        // let mut current=self.get_item(addr).unwrap().into_dir_item();
        let mut addr = addr;
        let zero: DirRawItem = DirRawItem {
            flag: DirRawItem::NON_USE_FLAG,
            reserved: [0; 31],
        };
        let mut short_flag = false;
        loop {
            match self.get_flag(addr) {
                DirRawItem::NON_USE_FLAG => {
                    break;
                }
                DirRawItem::SHORT_FLAG => {
                    if short_flag {
                        break;
                    }
                    short_flag = true;
                    self.insert_item(zero, addr);
                    addr.step();
                }
                DirRawItem::LONG_END_FLAG => {
                    if !short_flag {
                        println!("set_empty_item函数输入的头部不是shortitem！");
                        break;
                    }
                    self.insert_item(zero, addr);
                    break;
                }
                _ => {
                    if !short_flag {
                        println!("set_empty_item函数输入的头部不是shortitem！");
                        break;
                    }
                    self.insert_item(zero, addr);
                    addr.step();
                    continue;
                }
            }
        }
    }

    pub fn item_status(&mut self) {
        let range = ItemAddrRange::new(ItemAddr::new(0), ItemAddr::new(self.num));
        let mut count = 0;
        for i in range.iter() {
            count += 1;
            if (count % 8 == 0) {
                println!();
            }

            let flag = self.get_flag(i);
            match flag {
                DirRawItem::NON_USE_FLAG => {
                    print!("None\t");
                }
                DirRawItem::SHORT_FLAG => {
                    print!("Short\t");
                }
                DirRawItem::LONG_END_FLAG => {
                    print!("End\t");
                }
                _ => {
                    print!("Long\t");
                }
            }
        }
    }

    pub fn dir_empty(&mut self) -> bool {
        let range = ItemAddrRange::new(ItemAddr::new(0), ItemAddr::new(self.num));
        for i in range.iter() {
            let flag = self.get_flag(i);
            match flag {
                DirRawItem::SHORT_FLAG => {
                    return false;
                }
                _ => {
                    continue;
                }
            }
        }
        return true;
    }
}
///目录项的地址
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ItemAddr {
    addr: u32,
}

impl ItemAddr {
    fn new(addr: u32) -> Self {
        ItemAddr { addr }
    }

    fn get_offset(&self) -> u32 {
        self.addr * DirRawItem::ITEM_SIZE as u32 + ZDirPack::PACK_SIZE as u32 /*+ DirServant::HEAD_RESERVE_SIZE as u32*/
    }

    fn get_addr(&self) -> u32 {
        self.addr
    }

    pub fn step(&mut self) {
        self.addr += 1;
    }
}

impl Add<u32> for ItemAddr {
    type Output = ItemAddr;
    fn add(self, rhs: u32) -> Self::Output {
        Self {
            addr: self.addr + rhs,
        }
    }
}

impl Sub<u32> for ItemAddr {
    type Output = ItemAddr;
    fn sub(self, rhs: u32) -> Self::Output {
        Self {
            addr: self.addr - rhs,
        }
    }
}

pub struct ItemAddrRange {
    start: ItemAddr,
    end: ItemAddr,
}

impl ItemAddrRange {
    pub fn new(start: ItemAddr, end: ItemAddr) -> ItemAddrRange {
        ItemAddrRange { start, end }
    }

    pub fn iter(&self) -> ItemAddrRangeIter {
        ItemAddrRangeIter::new(self.start, self.end)
    }
}

#[derive(Debug)]
pub struct ItemAddrRangeIter {
    current: ItemAddr,
    end: ItemAddr,
}

impl ItemAddrRangeIter {
    fn new(current: ItemAddr, end: ItemAddr) -> ItemAddrRangeIter {
        ItemAddrRangeIter { current, end }
    }
}

impl Iterator for ItemAddrRangeIter {
    type Item = ItemAddr;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.current != self.end) {
            let temp = self.current;
            self.current = self.current + 1 as u32;
            Some(temp)
        } else {
            None
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DirRawItem {
    flag: u8,
    reserved: [u8; 31],
}

impl From<ShortDirItem> for DirRawItem {
    fn from(value: ShortDirItem) -> Self {
        let flag = value.get_flag();
        let reserved = unsafe { transmute::<ItemData, [u8; 31]>(value.data) };
        Self { flag, reserved }
    }
}

impl From<LongDirItem> for DirRawItem {
    fn from(value: LongDirItem) -> Self {
        let flag = value.flag;
        let reserved = value.data;
        Self { flag, reserved }
    }
}

impl DirRawItem {
    const ITEM_SIZE: usize = 32;
    const SHORT_FLAG: u8 = 0b0000_0001;
    const LONG_END_FLAG: u8 = 0b1111_1111;
    const NON_USE_FLAG: u8 = 0b0000_0000;
    pub fn new(di: DirItem) -> DirRawItem {
        match di {
            //todo: 这里有一个很严重的问题，单纯的transmute可能会导致严重的后果
            DirItem::Long(item) => Self::from(item),
            DirItem::Short(item) => Self::from(item),
            DirItem::None => {
                println!("您尝试使用空DirItem生成DirRawItem，这是不允许的");
                panic!()
            }
        }
    }

    pub fn into_dir_item(self) -> DirItem {
        match self.flag {
            Self::SHORT_FLAG => DirItem::Short(ShortDirItem::from(self)),
            Self::NON_USE_FLAG => DirItem::None,
            _ => DirItem::Long(LongDirItem::from(self)),
        }
    }

    pub fn into_u8(self) -> [u8; Self::ITEM_SIZE] {
        unsafe { transmute::<DirRawItem, [u8; Self::ITEM_SIZE]>(self) }
    }

    pub fn from_u8(data: [u8; Self::ITEM_SIZE]) -> DirRawItem {
        unsafe { transmute::<[u8; Self::ITEM_SIZE], DirRawItem>(data) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DirItem {
    Long(LongDirItem),
    Short(ShortDirItem),
    None,
}

impl DirItem {
    fn get_name(self) -> Result<Vec<u8>, ()> {
        match self {
            DirItem::Long(item) => Ok(item.get_name_zone()),
            DirItem::Short(item) => Ok(item.get_name_zone()),
            DirItem::None => Err(()),
        }
    }

    fn get_flag(self) -> Result<u8, ()> {
        match self {
            DirItem::Long(item) => Ok(item.get_flag()),
            DirItem::Short(item) => Ok(item.get_flag()),
            DirItem::None => Ok(DirRawItem::NON_USE_FLAG),
        }
    }
}

///用于根据一个字符串来产生DirItem的串
/// 它的用法是：输入一个字符串和入口，然后生成一个待写入的迭代器，迭代器每迭代一次，都会吐出一个你要写入的项
#[derive(Debug)]
pub struct DirItemGenerateIter {
    list: Vec<DirItem>,
    count: usize,
    length: usize,
}

impl DirItemGenerateIter {
    pub fn new(name: &str, entry: BlockAddr, file_type: FileType) -> DirItemGenerateIter {
        let target_num = name.len();
        let mut finish_num: usize = 0;
        let ans: Vec<DirItem> = vec![];

        let si = DirItem::Short(ShortDirItem::from_name_entry(name, entry, file_type));
        let mut list: Vec<DirItem> = vec![];
        list.push(si);
        finish_num = ItemData::SHORT_NAME_SIZE;
        let mut flag = 1;
        while (target_num > finish_num) {
            flag += 1;

            let this_flag = if (target_num > finish_num + 31) {
                flag
            } else {
                DirRawItem::LONG_END_FLAG
            };
            let temp_long = LongDirItem::from_name_flag(name, &finish_num, this_flag);
            finish_num += 31;
            list.push(DirItem::Long(temp_long));
        }
        DirItemGenerateIter {
            list: list,
            count: 0,
            length: flag as usize,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl Iterator for DirItemGenerateIter {
    type Item = DirItem;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.count < self.length) {
            let temp = Some(self.list[self.count]);
            self.count += 1;
            temp
        } else {
            None
        }
    }
}

///long目录项被用于存储目录名，如果short不够的话就会生成long，每个long都可以存储31个字节
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LongDirItem {
    flag: u8,
    data: [u8; 31],
}

impl From<DirRawItem> for LongDirItem {
    fn from(value: DirRawItem) -> Self {
        Self {
            flag: value.flag,
            data: value.reserved,
        }
    }
}

impl LongDirItem {
    pub fn from_name_flag(name: &str, offset: &usize, my_flag: u8) -> LongDirItem {
        let mut offset = *offset;
        let mut data: [u8; 31] = [0; 31];
        let count = if name.len() - offset > 31 {
            offset + 31
        } else {
            name.len()
        };
        let name = name.as_bytes();
        for i in offset..count {
            data[i - offset] = name[i];
        }
        LongDirItem {
            flag: my_flag,
            data,
        }
    }
    pub fn get_name_zone(&self) -> Vec<u8> {
        let mut ans: Vec<u8> = vec![];
        for i in self.data {
            if i == 0 {
                break;
            }
            ans.push(i);
        }
        ans
    }

    pub fn get_flag(self) -> u8 {
        self.flag
    }
}

///short字符串存储着文件入口以及一部分的名字，名字字段有27个字节
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ShortDirItem {
    flag: u8,
    data: ItemData,
}

impl From<DirRawItem> for ShortDirItem {
    fn from(value: DirRawItem) -> Self {
        let flag = value.flag;
        let data = unsafe { transmute::<[u8; 31], ItemData>(value.reserved) };

        Self { flag, data }
    }
}

impl ShortDirItem {
    pub fn new(flag: u8, data: ItemData) -> ShortDirItem {
        ShortDirItem { flag, data }
    }

    pub fn from_name_entry(name: &str, entry: BlockAddr, file_type: FileType) -> ShortDirItem {
        let flag: u8 = DirRawItem::SHORT_FLAG;
        let data = ItemData::from_name_entry(name, entry, file_type);
        ShortDirItem { flag, data }
    }

    pub fn get_name_zone(&self) -> Vec<u8> {
        let mut ans: Vec<u8> = vec![];
        for i in self.data.get_name() {
            if (i == 0) {
                break;
            }
            ans.push(i);
        }
        ans
    }

    pub fn entry(&self) -> BlockAddr {
        unsafe { transmute::<[u8; 4], BlockAddr>(self.data.addr) }
    }

    pub fn get_flag(self) -> u8 {
        self.flag
    }

    pub fn get_block(&self) -> BlockAddr {
        self.data.get_addr()
    }

    pub fn get_type(&self) -> FileType {
        self.data.file_type
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ItemData {
    addr: [u8; 4],
    file_type: FileType,
    reserved: u8,
    name: [u8; ItemData::SHORT_NAME_SIZE],
}

impl ItemData {
    pub const SHORT_NAME_SIZE: usize = 25;
    const DATA_SIZE: usize = 31;
    fn new(data: [u8; Self::DATA_SIZE]) -> ItemData {
        unsafe { transmute::<[u8; Self::DATA_SIZE], Self>(data) }
    }

    pub fn from_name_entry(name: &str, entry: BlockAddr, file_type: FileType) -> ItemData {
        if (name.len() == 0) {
            println!("输入name的长度为0！");
            panic!();
        } else {
            let addr: [u8; 4] = unsafe { transmute::<BlockAddr, [u8; 4]>(entry) };
            let mut list: [u8; ItemData::SHORT_NAME_SIZE] = [0; ItemData::SHORT_NAME_SIZE];
            let name = name.as_bytes();
            let count = if name.len() > ItemData::SHORT_NAME_SIZE {
                ItemData::SHORT_NAME_SIZE
            } else {
                name.len()
            };
            for i in 0..count {
                list[i] = name[i];
            }
            ItemData {
                addr,
                name: list,
                file_type,
                reserved: 0,
            }
        }
    }

    fn get_name(&self) -> [u8; ItemData::SHORT_NAME_SIZE] {
        self.name
    }

    fn get_addr(&self) -> BlockAddr {
        unsafe { transmute::<[u8; 4], BlockAddr>(self.addr) }
    }
}

#[cfg(test)]
#[test]
fn test_insert() {
    // use crate::sys_utility::file::zdir::ZDir;

    // let mut zd = ZDir::open(BlockAddr { addr: 83 }).unwrap();
    // println!("{:?}", zd);

    // let item = ShortDirItem::new(
    //     DirRawItem::SHORT_FLAG,
    //     ItemData {
    //         addr: [0; 4],
    //         name: [66; 27],
    //     },
    // );
    // let item = DirRawItem::new(DirItem::Short(item));
    // println!("{:?}", item);

    // zd.servant.insert_item(item, ItemAddr { addr: 1 });
    // let a = zd.servant.get_item(ItemAddr { addr: 1 }).unwrap();

    // println!("{:?}", a);
    // zd.close();
}

#[test]
fn test_diritemaddrrangeiter() {
    let a = ItemAddrRange::new(ItemAddr { addr: 0 }, ItemAddr { addr: 10 });
    let b = a.iter();
    println!("{:?}", b);
    for i in b {
        println!("{:?}", i);
    }
}

#[test]
fn test_api() {
    let a = "asldjalsjdlakjds";
    println!("{}", a.len());
}

#[test]
fn test_diritem_generator() {
    let gen = DirItemGenerateIter::new("hello world", BlockAddr { addr: 100 }, FileType::Dir);
    for i in gen {
        println!("{:?}", i);
    }
}

#[test]
fn test_get_name() {
    let raw = RawFile::open(BlockAddr { addr: 97 }).unwrap();
    let mut serve = DirServant::new(raw, 30);
    // serve.new_dir_item("ABCD", BlockAddr { addr: 251 });
    // serve.new_dir_item("ABCDd", BlockAddr { addr: 251 });
    // serve.new_dir_item("ABCDd", BlockAddr { addr: 251 });
    // serve.new_dir_item("ABCDd", BlockAddr { addr: 251 });
    // serve.new_dir_item("ABCDd", BlockAddr { addr: 251 });

    let ans = serve.get_name(ItemAddr { addr: 8 });
    println!("{:?}", ans);
    serve.command_ls();
}
