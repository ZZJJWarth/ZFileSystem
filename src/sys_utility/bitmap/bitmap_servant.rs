use std::ops::Add;

use crate::sys_utility::block::file_writer;

use super::super::{
    addr::addr::{Addr, BlockAddr, BlockCount, WordAddrCount},
    block::{file_reader::FileReader, file_writer::FileWriter},
    config::config::{BLOCK_SIZE, NON_OCCUPY_NUM},
};
use super::block_bit_map::BlockBitmap;
///位图仆人，给位图提供访问磁盘的服务
#[derive(Debug)]
pub struct BitmapServant {
    block_entry: BlockAddr,
    f_writer: FileWriter,
    f_reader: FileReader,
}

impl BitmapServant {
    ///输入位图的入口，位图仆人
    pub fn new(block_entry: BlockAddr) -> BitmapServant {
        BitmapServant {
            block_entry,
            f_writer: FileWriter::new(file_writer::IoOption::Bitmap),
            f_reader: FileReader::new(file_writer::IoOption::Bitmap),
        }
    }

    ///找到文件的最大block
    pub fn file_max_block(&self) -> BlockAddr {
        //向下调用接口
        let u8_len = self.f_writer.get_file_len() as u32;
        BlockAddr::new(u8_len / BLOCK_SIZE)
    }
    ///给定一个blockAddr，将对应的Bitmap修改成未占用
    pub fn set_non_occupied(&mut self, block_addr: BlockAddr) {
        let offset = BlockOffset::new(block_addr) + self.block_entry;
        self.f_writer.bitmap_write(offset, NON_OCCUPY_NUM);
    }
    ///给定一个blockAddr,将对应的Bitmap修改成指定的BlockAddr
    pub fn set_value(&mut self, block_addr: BlockAddr, value: BlockAddr) {
        // println!("set_value in {:?} with {:?}", block_addr, value);
        // println!("in{:?},{:?}",block_addr,value);
        let offset = BlockOffset::new(block_addr) + self.block_entry;
        // println!("write:{:?}", value);
        self.f_writer.bitmap_write(offset, value);
        // println!("Exit");
    }
    ///check一个块是否是空闲的
    pub fn check_block_empty(&mut self, block: BlockAddr) -> bool {
        let offset = BlockOffset::new(block) + self.block_entry;
        let read = self.f_reader.bitmap_read(offset);
        // println!("{:?}",read==NON_OCCUPY_NUM);
        if (read == NON_OCCUPY_NUM) {
            true
        } else {
            false
        }
    }
    ///在表中读一个块，得到下一个块的地址
    pub fn read_a_block(&mut self, block: BlockAddr) -> BlockAddr {
        let offset = BlockOffset::new(block) + self.block_entry;
        let read = self.f_reader.bitmap_read(offset);
        read
    }
}
#[derive(Debug)]
pub struct BlockOffset {
    b_offset: BlockCount,
    w_offset: WordAddrCount,
}

impl BlockOffset {
    ///为块地址服务的Offset，接收一个绝对块地址，返回一个针对Bitmap有效的Offset，指出应该放入第几的块的哪里
    pub fn new(block: BlockAddr) -> BlockOffset {
        let rem = (BLOCK_SIZE / 4);
        BlockOffset {
            b_offset: block / rem,
            w_offset: (block % rem) * 4,
        }
    }

    pub fn addr_offset(&self) -> Addr {
        self.b_offset + self.w_offset
    }
}

impl Add<BlockAddr> for BlockOffset {
    type Output = BlockOffset;
    fn add(self, rhs: BlockAddr) -> Self::Output {
        BlockOffset {
            b_offset: self.b_offset + rhs,
            w_offset: self.w_offset,
        }
    }
}

#[cfg(test)]

// #[test]
fn test1() {
    let mut a = BlockBitmap::new(BlockAddr::new(0), 10, 1);
    a.init();
    let mut ser = BitmapServant::new(BlockAddr { addr: 0 });
    a.get_free_block();
    println!("{:?}", ser.read_a_block(BlockAddr::new(0)));
    println!("{:?}", ser.read_a_block(BlockAddr::new(1)));
    println!("{:?}", ser.read_a_block(BlockAddr::new(2)));
    println!("{:?}", ser.read_a_block(BlockAddr::new(3)));
    println!("{:?}", ser.read_a_block(BlockAddr::new(4)));
    println!("{:?}", ser.read_a_block(BlockAddr::new(5)));
    println!("{:?}", ser.read_a_block(BlockAddr::new(6)));
    println!("{:?}", ser.read_a_block(BlockAddr::new(7)));
    println!("{:?}", ser.read_a_block(BlockAddr::new(8)));
}

#[test]
fn test2_init() {
    let mut bitmap = BlockBitmap::new(BlockAddr { addr: 1 }, 102400, 401);
    println!("{:?}", bitmap);
    bitmap.init();
}
