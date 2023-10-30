use super::super::{
    addr::addr::{Addr, BlockAddr},
    bitmap::block_bit_map::BlockBitmap,
    block::file_reader::FileReader,
    block::file_writer::FileWriter,
    config::config::BLOCK_SIZE,
};

// static mut TEST_BITMAP:BlockBitmap=BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2);

use std::ops::{Add, Div, Rem, Sub};
#[derive(Debug, Clone, Copy)]
pub struct BlockServant {
    pub entry: BlockAddr,
}

impl BlockServant {
    pub fn new(entry: BlockAddr) -> BlockServant {
        BlockServant { entry }
    }

    pub fn write(&self, offset: u32, data: &Vec<u8>, size: u32) -> Result<(), ()> {
        // println!("offset:{}",offset);
        let mut fw = FileWriter::new(super::file_writer::IoOption::Other(BLOCK_SIZE));
        let mut ptr = data.as_slice();

        let start = VirtualAddr { addr: offset };
        let range = VirtualRange::with_size(start, size);
        let n = range.relative_start_block_gap();
        // println!("n:{}", n);
        //todo:这个bitmap是测试使用的，真正运行的时候应该是用应该static的bitmap
        let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
        let mut now_block = self.entry;

        for i in 0..n {
            now_block = bit_map.get_content(now_block);
        }
        // println!("now_block:{:?}",now_block);
        let mut index = 0;
        for i in range.iter() {
            // println!("{:?}",i);
            let next_block = bit_map.get_content(now_block);

            let bo_range = BlockServantOffsetRange::new(now_block, i);
            now_block = next_block;
            let l = bo_range.get_len();
            let dp = DataPack::new(ptr, index, l);
            index = index + l as usize;
            // println!("fw.write: bo_range:{:?},dp:{:?}",bo_range,dp);
            fw.write(bo_range, dp);
            // // println!("{:?}", bo_range);
            // for i in 0..n {
            // now_block = bit_map.get_content(now_block);
            // println!("{:?}",now_block);
            // }
        }
        Ok(())
    }

    pub fn read(&self, offset: u32, data: &mut Vec<u8>, size: u32) -> Result<(), ()> {
        let mut fr = FileReader::new(super::file_writer::IoOption::Other(BLOCK_SIZE));
        let start = VirtualAddr { addr: offset };
        let range = VirtualRange::with_size(start, size);

        fr.read(range, data, self.entry);

        Ok(())
    }

    pub fn read_check(&self, file_len: u32, offset: u32, size: u32) -> Result<(), ()> {
        if (file_len >= offset + size) {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn write_check(&self, file_max_len: u32, offset: u32, size: u32) -> Result<u32, ()> {
        if (file_max_len >= offset + size) {
            Ok(file_max_len)
        } else {
            //todo:这个bitmap是测试使用的，真正运行的时候应该是用应该static的bitmap
            let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
            let mut now_len = file_max_len;
            let mut last_block = bit_map.find_final_block(self.entry).unwrap();
            // println!("last_block:{:?}",last_block);
            // let mut count=10;
            while (now_len < offset + size) {
                // count=count-1;

                // if(count==0){
                //     break;
                // }
                last_block = bit_map.add_block(last_block);

                now_len = now_len + BLOCK_SIZE;
                // println!("now_len={},offset+size={}",now_len,offset+size);
            }
            Ok(now_len)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VirtualAddr {
    addr: u32,
}

impl Rem<u32> for VirtualAddr {
    type Output = u32;
    fn rem(self, rhs: u32) -> Self::Output {
        self.addr % rhs
    }
}

impl Sub<VirtualAddr> for VirtualAddr {
    type Output = VirtualAddrCount;
    fn sub(self, rhs: VirtualAddr) -> Self::Output {
        VirtualAddrCount {
            num: self.addr - rhs.addr,
        }
    }
}

impl Div<u32> for VirtualAddr {
    type Output = u32;
    fn div(self, rhs: u32) -> Self::Output {
        self.addr / rhs
    }
}

impl Add<BlockAddr> for VirtualAddr {
    type Output = Addr;
    fn add(self, rhs: BlockAddr) -> Self::Output {
        Addr::new(self.addr + rhs.get_raw_num() * BLOCK_SIZE)
    }
}

impl VirtualAddr {
    pub fn new(num: u32) -> VirtualAddr {
        VirtualAddr { addr: num }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct VirtualAddrCount {
    num: u32,
}

impl VirtualAddrCount {
    fn new(num: u32) -> VirtualAddrCount {
        VirtualAddrCount { num }
    }

    fn len(&self) -> u32 {
        self.num
    }
}
#[derive(Debug, Clone, Copy)]
///本结构的用处是：将文件的虚拟offset转换成绝对地址（Addr）
struct BlockServantOffset {
    block: BlockAddr,
    offset: VirtualAddr,
}

impl BlockServantOffset {
    pub fn new(block: BlockAddr, offset: VirtualAddr) -> BlockServantOffset {
        BlockServantOffset { block, offset }
    }

    pub fn to_addr(self) -> Addr {
        self.offset + self.block
    }
}
#[derive(Debug, Clone, Copy)]
///可以直接向下提供的结构体，每次都是一块中间的范围,块地址的装载就在这里
pub struct BlockServantOffsetRange {
    block_entry: BlockAddr,
    v_range: VirtualRange,
}

impl BlockServantOffsetRange {
    pub fn new(block_entry: BlockAddr, v_range: VirtualRange) -> BlockServantOffsetRange {
        BlockServantOffsetRange {
            block_entry,
            v_range,
        }
    }

    pub fn get_block_entry(&self) -> BlockAddr {
        self.block_entry
    }

    pub fn get_start(&self) -> VirtualAddr {
        self.v_range.start
    }

    pub fn get_end(&self) -> VirtualAddr {
        self.v_range.end
    }

    pub fn get_len(&self) -> u32 {
        self.v_range.len.len()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VirtualRange {
    start: VirtualAddr,
    end: VirtualAddr,
    len: VirtualAddrCount,
}

impl VirtualRange {
    pub fn new(start: VirtualAddr, end: VirtualAddr) -> VirtualRange {
        let len = end - start;
        VirtualRange { start, end, len }
    }

    pub fn iter(&self) -> VirtualRangeIterator {
        VirtualRangeIterator::new(self.start, self.end)
    }

    pub fn with_size(start: VirtualAddr, size: u32) -> VirtualRange {
        VirtualRange::new(
            start,
            VirtualAddr {
                addr: start.addr + size,
            },
        )
    }

    pub fn relative_start_block_gap(&self) -> u32 {
        self.start.addr / BLOCK_SIZE as u32
    }
}

pub struct VirtualRangeIterator {
    current: VirtualAddr,
    end: VirtualAddr,
}

impl VirtualRangeIterator {
    pub fn new(start: VirtualAddr, end: VirtualAddr) -> VirtualRangeIterator {
        VirtualRangeIterator {
            current: start,
            end,
        }
    }

    pub fn get_range(&self) -> Option<VirtualRange> {
        let i = self.current / BLOCK_SIZE + 1;
        // println!("{}",i);
        let i = i * BLOCK_SIZE;
        if (i < self.end.addr) {
            Some(VirtualRange::new(self.current, VirtualAddr { addr: i }))
        } else if (self.end.addr <= self.current.addr) {
            None
        } else {
            Some(VirtualRange::new(self.current, self.end))
        }
    }

    pub fn terminate(&self) -> bool {
        let i = self.current / BLOCK_SIZE + 1;
        let i = i * BLOCK_SIZE;
        if (i > self.end.addr) {
            true
        } else {
            false
        }
    }
}

impl Iterator for VirtualRangeIterator {
    type Item = VirtualRange;
    fn next(&mut self) -> Option<Self::Item> {
        // todo!()
        let ans = self.get_range();
        self.current = VirtualAddr::new((self.current / BLOCK_SIZE + 1) * BLOCK_SIZE);
        ans
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DataPack<'a> {
    pub write_in: &'a [u8],
    pub index: usize,
    pub len: u32,
}

impl<'a> DataPack<'a> {
    pub fn new(write_in: &'a [u8], index: usize, len: u32) -> DataPack {
        DataPack {
            write_in,
            index,
            len,
        }
    }
}

#[cfg(test)]
// #[test]
fn test() {
    let block_entry = BlockAddr::new(10);
    let vaddr = VirtualAddr::new(10);
    let blockset = BlockServantOffset::new(block_entry, vaddr);
    assert_eq!(blockset.to_addr(), Addr::new(10250));
}

// #[test]
fn test1() {
    let s = VirtualAddr::new(1023);
    let e = VirtualAddr::new(1024);
    let r = VirtualRange::with_size(s, 2);
    let i = r.iter();
    for j in i {
        println!("{:?}", j);
    }
}
//这个test
// #[test]
fn test2() {
    let s = BlockServant::new(BlockAddr { addr: 1 });
    let mut buff: Vec<u8> = vec![];
    s.write(
        1016,
        &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 3, 2, 1],
        16,
    );
    s.read(1016, &mut buff, 16);
    println!("{:?}", buff);
}
