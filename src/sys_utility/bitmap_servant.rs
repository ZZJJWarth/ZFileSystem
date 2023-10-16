use std::ops::Add;

use super::addr::{BlockAddr, BlockCount, Addr, WordAddrCount};
use super::config::{BLOCK_SIZE, NON_OCCUPY_NUM};
use super::file_writer::FileWriter;
#[derive(Debug)]
pub struct BitmapServant{
    block_entry:BlockAddr,
    f_writer:FileWriter
}

impl BitmapServant{
    pub fn new(block_entry:BlockAddr)->BitmapServant{
        BitmapServant{
            block_entry,
            f_writer:FileWriter::new(super::file_writer::WriterOption::Bitmap)
        }
    }

///找到文件的最大block
    pub fn file_max_block(&self) -> BlockAddr {  //向下调用接口
        let u8_len=self.f_writer.get_file_len() as u32;
        BlockAddr::new(u8_len/BLOCK_SIZE)
    }
///给定一个blockAddr，将对应的Bitmap修改成未占用
    pub fn set_non_occupied(&mut self,block_addr:BlockAddr){       
        let offset=BlockOffset::new(block_addr)+self.block_entry;
        self.f_writer.bitmap_write(offset, NON_OCCUPY_NUM);
    }
///给定一个blockAddr,将对应的Bitmap修改成指定的BlockAddr
    pub fn set_value(&mut self,block_addr:BlockAddr,value:BlockAddr){
        let offset=BlockOffset::new(block_addr)+self.block_entry;
        self.f_writer.bitmap_write(offset, block_addr);
    }

}
#[derive(Debug)]
pub struct BlockOffset{
    b_offset:BlockCount,
    w_offset:WordAddrCount,
}

impl BlockOffset{
    fn new(block:BlockAddr)->BlockOffset{
        let rem=(BLOCK_SIZE/4);
        BlockOffset { b_offset: block/rem, w_offset:block%rem }
    }

    pub fn addr_offset(&self)->Addr{
        self.b_offset+self.w_offset
    }
}

impl Add<BlockAddr> for BlockOffset{
    type Output = BlockOffset;
    fn add(self, rhs: BlockAddr) -> Self::Output {
        BlockOffset{
            b_offset:self.b_offset+rhs,
            w_offset:self.w_offset
        }
    }
}

#[cfg(test)]

#[test]
fn test1(){
    
    let a=BlockOffset::new(BlockAddr::new(266));
    println!("{:?}",a);
}