use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    mem::transmute,
};

use crate::sys_utility::{block_servant::BlockServantOffsetRange, file_writer::AddrRange, config::BLOCK_SIZE};

use super::{
    addr::BlockAddr, bitmap_servant::BlockOffset, config::FILE_PATH, file_writer::IoOption, block_servant::VirtualRange, block_bit_map::BlockBitmap,
};
#[derive(Debug)]
pub struct FileReader {
    br: BufReader<File>,
}

impl FileReader {
    pub fn new(option: IoOption) -> FileReader {
        let f = File::open(FILE_PATH).unwrap();
        let br = match option {
            IoOption::Bitmap => BufReader::with_capacity(4, f),
            IoOption::Other(buf) => BufReader::with_capacity(buf as usize, f),
        };
        FileReader { br }
    }

    pub fn bitmap_read(&mut self, offset: BlockOffset) -> BlockAddr {
        let offset = offset.addr_offset();
        // println!("offset={:?}",offset);
        self.br
            .seek(std::io::SeekFrom::Start(offset.get_raw_num() as u64));
        self.br.fill_buf();
        let a = self.br.buffer().as_ptr();
        // println!("a={:?}",unsafe { *(a as *const BlockAddr) });
        let num: BlockAddr = unsafe { *(a as *const BlockAddr) };
        num
    }

    pub fn read(&mut self,range:VirtualRange,v:&mut Vec<u8>,block:BlockAddr){
        let n = range.relative_start_block_gap();
        //todo:这个bitmap是测试使用的，真正运行的时候应该是用应该static的bitmap
        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        let mut now_block = block;
        for i in 0..n {
            now_block = bit_map.get_content(now_block);
            // println!("{:?}",now_block);
        }
        for i in range.iter(){
            self.read_block(now_block);
            let mut block=self.br.buffer();
            let bi=BlockServantOffsetRange::new(now_block,i);
            let range=AddrRange::from_block_servant_range(bi);
            let start=range.start.get_raw_num()%BLOCK_SIZE;
            let mut end=range.end.get_raw_num()%BLOCK_SIZE;
            if(end==0){
                end=BLOCK_SIZE;
            }
            for i in start..end{
                v.push(block[i as usize]);
            }
            now_block=bit_map.get_content(now_block);
        }
    }

    fn read_block(&mut self, block: BlockAddr){
        let offset=block.addr*BLOCK_SIZE;
        self.br.seek(SeekFrom::Start(offset as u64));
        self.br.fill_buf();
    }
}

#[cfg(test)]

// #[test]
fn test1() {
    let mut fr = FileReader::new(IoOption::Bitmap);
    let addr = fr.bitmap_read(BlockOffset::new(BlockAddr { addr: 500 }));
    println!("{:?}", addr);
}
