use super::raw_file::RawFile;

///帮助zdir创建、删除、查找、遍历目录项
struct DirServant{
    raw:RawFile,
}

impl DirServant{
    pub fn new(raw:RawFile)->Self{
        Self{raw}
    }

    pub fn add
}
///目录项的地址
struct ItemAddr{
    addr:u32
}