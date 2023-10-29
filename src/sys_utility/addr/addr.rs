use std::convert::From;
use std::ops::{Add, Div, Mul, Rem, Sub, SubAssign};

use super::super::{bitmap::bitmap_servant::BlockOffset, config::config::BLOCK_SIZE};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Addr {
    addr: u32,
}

impl Addr {
    pub fn new(num: u32) -> Addr {
        Addr { addr: num }
    }

    pub fn set_addr(&mut self, num: u32) {
        self.addr = num;
    }

    pub fn get_raw_num(&self) -> u32 {
        self.addr
    }

    pub fn into_block_addr(&self) -> BlockAddr {
        BlockAddr {
            addr: self.addr / 1024,
        }
    }
}

impl Add for Addr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            addr: self.addr + rhs.addr,
        }
    }
}

impl Sub<Addr> for Addr {
    type Output = u32;
    fn sub(self, rhs: Self) -> Self::Output {
        self.addr - rhs.addr
    }
}

impl AsRef<u32> for Addr {
    fn as_ref(&self) -> &u32 {
        &self.addr
    }
}

impl From<BlockAddr> for Addr {
    fn from(value: BlockAddr) -> Self {
        Addr {
            addr: value.addr * 1024,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct BlockAddr {
    pub addr: u32,
}

impl BlockAddr {
    pub fn new(num: u32) -> BlockAddr {
        BlockAddr { addr: num }
    }

    pub fn into_addr(&self) -> Addr {
        Addr {
            addr: self.addr * 1024,
        }
    }

    pub fn get_raw_num(&self) -> u32 {
        self.addr
    }
}

impl Rem<u32> for BlockAddr {
    type Output = WordAddrCount;
    fn rem(self, rhs: u32) -> Self::Output {
        let num = self.addr % rhs;
        WordAddrCount::new(num)
    }
}

impl Div<u32> for BlockAddr {
    type Output = BlockCount;
    fn div(self, rhs: u32) -> Self::Output {
        let num = self.addr / rhs;
        BlockCount::new(num)
    }
}

impl Add<u32> for BlockAddr {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Self {
            addr: self.addr + rhs,
        }
    }
}

impl Add<Addr> for BlockAddr {
    type Output = Addr;
    fn add(self, rhs: Addr) -> Self::Output {
        Addr {
            addr: self.addr * BLOCK_SIZE + rhs.addr,
        }
    }
}

impl Add for BlockAddr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            addr: self.addr + rhs.addr,
        }
    }
}

impl From<Addr> for BlockAddr {
    fn from(value: Addr) -> Self {
        BlockAddr {
            addr: value.addr / 1024,
        }
    }
}
#[derive(Debug)]
pub struct BlockRange {
    start: BlockAddr,
    end: BlockAddr,
}

impl BlockRange {
    pub fn new(start: BlockAddr, end: BlockAddr) -> BlockRange {
        BlockRange { start, end }
    }

    pub fn iter(&self) -> BlockRangeIter {
        BlockRangeIter {
            current: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

pub struct BlockRangeIter {
    current: BlockAddr,
    end: BlockAddr,
}

impl Iterator for BlockRangeIter {
    type Item = BlockAddr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let ret = self.current.clone();
            self.current = self.current + BlockAddr::new(1);
            Some(ret)
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BlockCount {
    num: u32,
}

impl Add<WordAddrCount> for BlockCount {
    type Output = Addr;
    fn add(self, rhs: WordAddrCount) -> Self::Output {
        let num = self.num * BLOCK_SIZE + rhs.num;
        Addr::new(num)
    }
}

impl Add<BlockAddr> for BlockCount {
    type Output = BlockCount;
    fn add(self, rhs: BlockAddr) -> Self::Output {
        BlockCount {
            num: self.num + rhs.get_raw_num(),
        }
    }
}

impl BlockCount {
    pub fn new(num: u32) -> BlockCount {
        BlockCount { num }
    }

    pub fn reduce(&mut self) -> bool {
        if (self.num > 0) {
            self.num = self.num - 1;
            true
        } else {
            false
        }
    }
}
#[derive(Debug)]
struct WordAddr {
    addr: u32,
}

impl WordAddr {
    pub fn new(num: u32) -> WordAddr {
        WordAddr { addr: num }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WordAddrCount {
    num: u32,
}

impl Mul<u32> for WordAddrCount {
    type Output = WordAddrCount;
    fn mul(self, rhs: u32) -> Self::Output {
        WordAddrCount {
            num: self.num * rhs,
        }
    }
}

impl WordAddrCount {
    pub fn new(num: u32) -> WordAddrCount {
        WordAddrCount { num }
    }
}
