use std::mem::transmute;

use crate::sys_utility::addr::addr::BlockAddr;

use super::raw_file::RawFile;

///帮助zdir创建、删除、查找、遍历目录项
#[derive(Debug)]
pub struct DirServant{
    raw:RawFile,
    num:u32,
}

impl DirServant{
    pub fn new(raw:RawFile,num:u32)->Self{
        Self{raw,num}
    }

    pub fn get_block_entry(&self)->BlockAddr{
        self.raw.get_block_entry()
    }

    pub fn get_item_num(&self)->u32{
        self.num
    }

    pub fn file(&mut self)->&mut RawFile{
        &mut self.raw
    }
    
    ///傻瓜式地插入一个Item
    fn insert_item(&mut self,item:DirRawItem,addr:ItemAddr)->Result<(),()>{
        let mut buf=item.into_u8().to_vec();
        self.raw.write(addr.get_offset(), &mut buf, DirRawItem::ITEM_SIZE as u32)
    }
    ///傻瓜式地读取一个Item
    fn get_item(&mut self,addr:ItemAddr)->Result<DirRawItem,()>{
        let mut buf:Vec<u8>=vec![];
        self.raw.read(addr.get_offset(), &mut buf, DirRawItem::ITEM_SIZE as u32)?;
        let mut temp:[u8;DirRawItem::ITEM_SIZE as usize]=[0;DirRawItem::ITEM_SIZE as usize];
        for i in 0..DirRawItem::ITEM_SIZE as usize{
            temp[i]=buf[i];
        }
        Ok(DirRawItem::from_u8(temp))
    }

    
    // pub fn add
}
///目录项的地址
pub struct ItemAddr{
    addr:u32
}

impl ItemAddr{
    fn new(addr:u32)->Self{
        ItemAddr { addr }
    }

    fn get_offset(&self)->u32{
        self.addr*DirRawItem::ITEM_SIZE as u32
    }
}
pub struct DirRawItem{
    flag:u8,
    reserved:[u8;31],    
}

impl DirRawItem{
    const ITEM_SIZE:usize=32;
    const SHORT_FLAG:u8=0b1111_1111;
    const NON_USE_FLAG:u8=0b0000_0000;
    pub fn new(di:DirItem)->DirRawItem{     
        match di{
            DirItem::Long(item)=>{
                unsafe{transmute::<LongDirItem,DirRawItem>(item)}
            },
            DirItem::Short(item)=>{
                unsafe{transmute::<ShortDirItem,DirRawItem>(item)
            }
        }
    }
}

    pub fn into_dir_item(self)->DirItem{
        match self.flag{
            Self::SHORT_FLAG=>{
                DirItem::Short(unsafe{transmute::<DirRawItem,ShortDirItem>(self)})
            },
            _=>{
                DirItem::Long(unsafe{transmute::<DirRawItem,LongDirItem>(self)})
            }
        }
    }

    pub fn into_u8(self)->[u8;Self::ITEM_SIZE]{
        unsafe{transmute::<DirRawItem,[u8;Self::ITEM_SIZE]>(self)}
    }

    pub fn from_u8(data:[u8;Self::ITEM_SIZE])->DirRawItem{
        unsafe{transmute::<[u8;Self::ITEM_SIZE],DirRawItem>(data)}
    }
}


pub enum DirItem{
    Long(LongDirItem),
    Short(ShortDirItem)
}
pub struct LongDirItem{
    flag:u8,
    data:[u8;31],
}



pub struct ShortDirItem{
    flag:u8,
    data:ItemData,
}

impl ShortDirItem{
    
}

struct ItemData{
    addr:[u8;4],
    name:[u8;27]
}

impl ItemData{
    const DATA_SIZE:usize=31;
    fn new(data:[u8;Self::DATA_SIZE])->ItemData{
        unsafe{transmute::<[u8;Self::DATA_SIZE],Self>(data)}
    }

    fn get_name(&self)->[u8;27]{
        self.name
    }

    fn get_addr(&self)->BlockAddr{
        unsafe{transmute::<[u8;4],BlockAddr>(self.addr)}
    }
}