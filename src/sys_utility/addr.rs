use std::ops::Add;
use std::convert::From;

#[derive(Debug)]
pub struct Addr{
    addr:u32
}

impl Addr{
    pub fn new(num:u32)->Addr{
        Addr { addr: num }
    }

    pub fn set_addr(&mut self,num:u32){
        self.addr=num;
    }

    pub fn get_raw_num(&self)->u32{
        self.addr
    }

    pub fn into_block_addr(&self)->BlockAddr
    {
        BlockAddr { addr: self.addr/1024 }
    }
}

impl Add for Addr{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
    Self{
        addr:self.addr+rhs.addr
    }
}
}

impl AsRef<u32> for Addr{
    fn as_ref(&self) -> &u32 {
        &self.addr
    }
}

impl From<BlockAddr> for Addr{
    fn from(value: BlockAddr) -> Self {
        Addr { addr: value.addr*1024 }
    }
}

#[derive(Debug,PartialEq, Eq,PartialOrd,Clone,Copy)]
pub struct BlockAddr{
    addr:u32,
}

impl BlockAddr{
    pub fn new(num:u32)->BlockAddr{
        BlockAddr { addr: num }
    }

    pub fn into_addr(&self)->Addr{
        Addr{addr:self.addr*1024}
    }

    pub fn get_raw_num(&self)->u32{
        self.addr
    }
}

impl Add<u32> for BlockAddr{
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Self{
            addr:self.addr+rhs
        }
    }
}

impl Add for BlockAddr{
        type Output=Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self{
                addr:self.addr+rhs.addr
            }
    }
}

impl From<Addr> for BlockAddr{
    fn from(value: Addr) -> Self {
        BlockAddr { addr: value.addr/1024 }
    }
}

pub struct BlockRange{
    start:BlockAddr,
    end:BlockAddr,
}

impl BlockRange{
    pub fn new(start:BlockAddr,end:BlockAddr)->BlockRange{
        BlockRange{
            start,
            end,
        }
    }

    pub fn iter(&self)->BlockRangeIter{
        BlockRangeIter{
            current:self.start.clone(),
            end:self.end.clone(),
    }
    
}
}

pub struct BlockRangeIter{
    current:BlockAddr,
    end:BlockAddr,
}

impl Iterator for BlockRangeIter{
    type Item=BlockAddr;
    fn next(&mut self)->Option<Self::Item>{
        if self.current<=self.end{
            let ret=self.current.clone();
            self.current=self.current+BlockAddr::new(1);
            Some(ret)
        }else{
            None
        }
}
}

// struct WordAddr{
//     addr:u32,
// }

// impl WordAddr{
//     pub fn new(num:u32)->WordAddr{
//         WordAddr { addr: num }
//     }
    
// }
