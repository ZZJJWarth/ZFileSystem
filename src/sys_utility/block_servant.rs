use super::{
    addr::{Addr, BlockAddr},
    config::BLOCK_SIZE,
};
use std::ops::{Add, Div, Rem, Sub};
pub struct BlockServant {
    entry: BlockAddr,
}

impl BlockServant {
    pub fn new(entry: BlockAddr) -> BlockServant {
        BlockServant { entry }
    }

    // pub fn
}
#[derive(Debug, Clone, Copy)]
struct VirtualAddr {
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
struct VirtualAddrCount {
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
///可以直接向下提供的结构体，每次都是一块中间的范围
struct BlockServantOffsetRange {
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

    
}

#[derive(Debug, Clone, Copy)]
struct VirtualRange {
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
}

struct VirtualRangeIterator {
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

#[cfg(test)]
#[test]
fn test() {
    let block_entry = BlockAddr::new(10);
    let vaddr = VirtualAddr::new(10);
    let blockset = BlockServantOffset::new(block_entry, vaddr);
    assert_eq!(blockset.to_addr(), Addr::new(10250));
}

#[test]
fn test1() {
    let s = VirtualAddr::new(1023);
    let e = VirtualAddr::new(1024);
    let r = VirtualRange::with_size(s, 2);
    let i = r.iter();
    for j in i {
        println!("{:?}", j);
    }
}
