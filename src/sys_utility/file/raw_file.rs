use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::mem::transmute;
use std::vec;

use super::super::{
    addr::addr::BlockAddr,
    bitmap::block_bit_map::BlockBitmap,
    block::block_servant::BlockServant,
    config::config::{BLOCK_SIZE, NON_OCCUPY_NUM},
};

use crate::sys_utility::bitmap::bitmap_servant;
use crate::sys_utility::config::config::FILE_PATH;

use super::metadata::Metadata;
use super::raw_f::{FileType, RawF};
const ZFILE_SIZE: usize = 16;
#[derive(Debug, Clone, Copy)]
pub struct RawFile {
    metadata: Metadata,
    block_servant: BlockServant,
    file_type: FileType,
}

impl RawF for RawFile {
    fn get_type(&self) -> super::raw_f::FileType {
        self.file_type
    }

    fn metadata(&self) -> Metadata {
        self.metadata
    }
}

impl RawFile {
    ///给定一个空闲块，返回一个File
    pub fn new(file_type: FileType, block_entry: BlockAddr) -> RawFile {
        let mut f = RawFile {
            metadata: Metadata::new(0, BLOCK_SIZE),
            block_servant: BlockServant::new(block_entry),
            file_type: file_type,
        };
        let mut buf: Vec<u8> = vec![];

        let data = RawFile::file_to_u8(f.clone());
        for i in data {
            buf.push(i);
        }
        f.raw_write(0, &buf, ZFILE_SIZE as u32);
        f
    }

    pub fn read(&mut self, offset: u32, buf: &mut Vec<u8>, size: u32) -> Result<(), ()> {
        let offset = offset + ZFILE_SIZE as u32;
        self.raw_read(offset, buf, size)
    }

    fn raw_read(&mut self, offset: u32, buf: &mut Vec<u8>, size: u32) -> Result<(), ()> {
        let len = self.metadata.get_file_len();
        match self.block_servant.read_check(len, offset, size) {
            Ok(()) => {
                self.block_servant.read(offset, buf, size);
                Ok(())
            }
            Err(()) => Err(()),
        }
    }

    pub fn write(&mut self, offset: u32, buf: &Vec<u8>, size: u32) -> Result<(), ()> {
        // println!("{:?}",buf);
        let offset = offset + ZFILE_SIZE as u32;
        self.raw_write(offset, buf, size)
    }

    fn raw_write(&mut self, offset: u32, buf: &Vec<u8>, size: u32) -> Result<(), ()> {
        let now_len = self.metadata.get_file_len();
        if now_len < offset {
            println!("偏移量不能大于文件长度");
            return Err(());
        }

        let max_len = self.metadata.get_max_len();
        let max_len = self
            .block_servant
            .write_check(max_len, offset, size)
            .unwrap();
        self.metadata.set_max_len(max_len);
        // println!("write:{{offset:{},size:{}}}",offset,size);
        // println!("self:{{{:?}}}",self);
        // println!("{:?}",buf);
        self.block_servant.write(offset, buf, size).unwrap();
        let max = if now_len < size + offset {
            size + offset
        } else {
            now_len
        };
        self.metadata.set_file_len(max);
        Ok(())
    }

    pub fn file_to_u8(file: RawFile) -> [u8; ZFILE_SIZE] {
        let file = unsafe { transmute::<RawFile, [u8; ZFILE_SIZE]>(file) };
        file
    }

    pub fn u8_to_file(v: [u8; ZFILE_SIZE]) -> RawFile {
        let v = unsafe { transmute::<[u8; ZFILE_SIZE], RawFile>(v) };
        v
    }

    pub fn open(block: BlockAddr) -> Result<RawFile, ()> {
        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        if bit_map.get_content(block) == NON_OCCUPY_NUM {
            println!("文件块为空");
            return Err(());
        } else {
            let offset = block.into_addr().get_raw_num();
            let f = File::open(FILE_PATH).unwrap();
            let mut br = BufReader::with_capacity(ZFILE_SIZE, f);
            br.seek(SeekFrom::Start(offset as u64));
            br.fill_buf();
            let buf = br.buffer();
            let mut fa: [u8; ZFILE_SIZE] = [0; ZFILE_SIZE];
            // println!("{:?}",offset);
            // println!("{:?}",buf);
            for i in 0..ZFILE_SIZE {
                fa[i] = buf[i];
            }
            return Ok(RawFile::u8_to_file(fa));
        }
    }

    pub fn close(&mut self) {
        let temp = RawFile::file_to_u8(self.clone());
        let mut buf = vec![];
        for i in temp {
            buf.push(i);
        }
        // println!("{:?}", self);
        self.raw_write(0, &buf, ZFILE_SIZE as u32);
    }

    pub fn reduce(&mut self, size: u32) {
        let mut i = self.metadata.get_file_len();
        if (i <= size + ZFILE_SIZE as u32) {
            self.del_init();
            self.metadata.set_file_len(ZFILE_SIZE as u32);
            self.metadata.set_max_len(BLOCK_SIZE as u32);
            return;
        }
        let mut count = self.metadata.get_max_len();
        let after_i = i - size;
        let mut r_block = after_i / BLOCK_SIZE;
        let n_block = i / BLOCK_SIZE;
        let block = n_block - r_block;
        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        for i in 0..block {
            count -= BLOCK_SIZE;
            bit_map.reduce_a_block(self.block_servant.entry);
        }
        self.metadata.set_file_len(after_i);
        self.metadata.set_max_len(count);
    }

    pub fn add_write(&mut self, buf: &Vec<u8>, size: u32) -> Result<(), ()> {
        let offset = self.metadata.get_file_len();
        self.raw_write(offset, buf, size)
    }

    pub fn del_init(&mut self) {
        let i = self.metadata.get_max_len() / BLOCK_SIZE;
        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        for j in 0..i {
            bit_map.reduce_a_block(self.block_servant.entry);
        }
    }

    pub fn get_block_entry(&self) -> BlockAddr {
        {
            self.block_servant.entry
        }
    }

    #[cfg(test)]
    // #[test]
    fn test_zfile() {
        use crate::sys_utility::bitmap::block_bit_map::BlockBitmap;

        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
                                                                           // bit_map.init();
                                                                           // println!("content a:{:?}", bit_map.get_content(BlockAddr { addr: 2 }));
        let a = bit_map.get_free_block().unwrap();
        let mut f = RawFile::new(FileType::File, a);
        // let mut f=ZFile::open(BlockAddr { addr: 28}).unwrap();
        println!("{:?}", f);
        // f.reduce(10);
        // println!("{:?}",f);
        // let buf:Vec<u8>=vec![1;1024];
        // f.write(0, &buf, 1024);

        // let buf:Vec<u8>=vec![2;10240];
        // f.write(4096, &buf, 10240);
        // println!("{:?}",f);
        // let mut buf2:Vec<u8>=vec![];
        // f.read(0, &mut buf2, 5120);
        // println!("{:?}",buf2);
        f.close();
        // let mut f=ZFile::open(BlockAddr { addr: 2 }).unwrap();
        // println!("{:?}",f);
    }

    // #[test]
    fn test_reduce() {
        use crate::sys_utility::bitmap::block_bit_map::BlockBitmap;

        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        let mut f = RawFile::open(BlockAddr { addr: 53 }).unwrap();

        let mut v: Vec<u8> = vec![1; 15000];
        // f.add_write(&v, 15000);
        // let mut a:Vec<u8>=vec![];
        // f.read(30000, &mut a, 10);
        // println!("{:?}",a);
        println!("{:?}", f);
        f.reduce(15000);
        println!("{:?}", f);
        f.close();
    }
}

#[test]
fn test1_open() {
    let f = RawFile::open(BlockAddr { addr: 82 }).unwrap();
    println!("{:?}", f);
    f.clone();
}
