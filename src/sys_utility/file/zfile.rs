use crate::{
    file_shell::root_file::error::FileSystemOperationError,
    sys_utility::{
        addr::addr::BlockAddr,
        bitmap::block_bit_map::BlockBitmap,
        super_block::unwarper::{get_bitmap, unwrap_bitmap},
    },
};

use super::{
    raw_f::RawF,
    raw_file::{RawFile, ZFILE_SIZE},
};
#[derive(Debug)]
pub struct ZFile {
    raw: RawFile,
}

impl ZFile {
    ///生成一个新的文件，若生成失败则返回错误
    pub fn new() -> Result<ZFile, FileSystemOperationError> {
        // let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        let mut bm = get_bitmap()?;

        let mut bit_map = unwrap_bitmap(&bm)?;
        let entry = bit_map.get_free_block().unwrap();
        drop(bit_map);
        Ok(ZFile {
            raw: RawFile::new(super::raw_f::FileType::File, entry),
        })
    }
    ///根据文件入口获得一个文件
    pub fn open(ba: BlockAddr) -> Result<ZFile, FileSystemOperationError> {
        Ok(ZFile {
            raw: RawFile::open(ba)?,
        })
    }
    ///关闭文件，这会更新文件的一致性
    pub fn close(&mut self) -> Result<(), FileSystemOperationError> {
        self.raw.close()
    }
    ///给定一个raw文件，生成一个文件并初始化
    pub fn init_raw(raw: RawFile) {
        let mut f = ZFile { raw };
        f.close();
    }
    ///面向用户的文字读函数，不需要考虑偏移情况
    pub fn char_read(&self, offset: u32, size: u32) -> Vec<char> {
        let mut buf: Vec<u8> = vec![];
        self.raw.read(offset, &mut buf, size);
        let mut ans: Vec<char> = vec![];
        for i in buf {
            ans.push(i as char);
        }
        ans
    }
    ///向文件中写入字符
    pub fn char_write(&mut self, offset: u32, size: u32, buf: Vec<char>) -> Result<(), FileSystemOperationError> {
        let mut b: Vec<u8> = vec![];
        for i in buf {
            b.push(i as u8);
        }
        self.raw.write(offset, &mut b, size)?;
        Ok(())
    }
    ///删除文件的字符
    pub fn reduce(&mut self, size: u32) {
        self.raw.reduce(size);
    }
    ///获得自己的入口地址
    pub fn get_block_entry(&self) -> BlockAddr {
        self.raw.get_block_entry()
    }   
    ///自我删除
    pub fn del(&mut self) {
        self.raw.del();
    }
    ///cat指令对应的函数，会将文件内容以字符串的形式返回
    pub fn cat(&self) -> String {
        let vec = self.char_read(0, self.raw.metadata().get_file_len() - ZFILE_SIZE as u32);
        vec.iter().collect()
    }
    ///write指令对应的函数，返回写入的字节数
    pub fn write(&mut self, content: String) -> Result<usize, FileSystemOperationError> {
        let buf = content.as_bytes().to_vec();
        // println!("{:?}",buf);
        self.raw.add_write(&buf, content.len() as u32)
    }
    ///从某个给定的文件中复制内容到self
    pub fn cp_from(&mut self, source: &ZFile) -> Result<(), FileSystemOperationError> {
        let length = source.raw.metadata().get_file_len() - ZFILE_SIZE as u32;
        let content = source.char_read(0, length);
        // println!("cp_from{:?}",length);
        // println!("{:?}",content);
        self.char_write(0, length, content);
        Ok(())
    }

}

impl Drop for ZFile {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
#[test]
fn test_zfile() {
    // let mut zf = ZFile::open(BlockAddr { addr: 4 })
    // println!("{:?}", zf);
    // // zf.reduce(25);
    // // zf.write(format!("hello"));
    // // let input=vec!['h','e','l','l','o'];
    // // zf.char_write(0, 5, input).unwrap();
    // // zf.reduce(1);
    // // let output = zf.cat();
    // println!("{}", output);
    // zf.close();
}
