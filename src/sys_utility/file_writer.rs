use std::{
    fs::File,
    io::{BufWriter, Seek, SeekFrom, Write},
    mem::transmute,
};

use super::{
    addr::{Addr, BlockAddr},
    bitmap_servant::BlockOffset,
    block_servant::{BlockServantOffsetRange, DataPack},
    config::{BLOCK_SIZE, FILE_PATH},
};

pub enum IoOption {
    Bitmap,
    Other(u32),
}
#[derive(Debug)]
pub struct FileWriter {
    bf: BufWriter<File>,
}

impl FileWriter {
    pub fn new(opt: IoOption) -> FileWriter {
        let f = File::options().write(true).open(FILE_PATH).unwrap();
        match opt {
            IoOption::Bitmap => {
                let mut bf = BufWriter::with_capacity(4 as usize, f);
                FileWriter { bf }
            }
            IoOption::Other(cap) => {
                let mut bf = BufWriter::with_capacity(cap as usize, f);
                FileWriter { bf }
            }
        }
    }

    pub fn bitmap_write(&mut self, b_off: BlockOffset, value: BlockAddr) {
        let addr = b_off.addr_offset();
        self.bf.seek(SeekFrom::Start(addr.get_raw_num() as u64));

        let buf = unsafe { transmute::<BlockAddr, [u8; 4]>(value) };
        // println!("{:?}", buf);
        self.bf.write(&buf);
        // println!("{:?}",self.bf);
        // println!("{:?}",buf);
    }

    pub fn get_file_len(&self) -> u64 {
        let f = File::open(FILE_PATH).unwrap();
        let mt = f.metadata().unwrap();
        mt.len()
    }

    pub fn write(&mut self, br: BlockServantOffsetRange, dp: DataPack) -> Result<(), ()> {
        let br = AddrRange::from_block_servant_range(br);
        let ptr = unsafe { dp.write_in as *const &[u8] };
        let a: *const &[u8] = ptr.cast();
        unsafe {
            self.bf.write(*ptr);
        }

        Ok(())
    }
}

#[cfg(test)]

// #[test]
fn test1() {
    use crate::sys_utility::file_reader::FileReader;

    let mut f = FileWriter::new(IoOption::Bitmap);
    f.bitmap_write(
        BlockOffset::new(BlockAddr { addr: 0 }),
        BlockAddr { addr: 66 },
    );
    let mut fr = FileReader::new(IoOption::Bitmap);
    let addr = fr.bitmap_read(BlockOffset::new(BlockAddr { addr: 0 }));
    println!("{:?}", addr);
}
#[derive(Debug)]
struct AddrRange {
    start: Addr,
    end: Addr,
    len: u32,
}

impl AddrRange {
    pub fn new(start: Addr, end: Addr) -> Self {
        let len = end - start;
        AddrRange { start, end, len }
    }

    pub fn from_block_servant_range(range: BlockServantOffsetRange) -> Self {
        let entry = range.get_block_entry();
        let start = range.get_start() + entry;
        let end = range.get_end() + entry;
        AddrRange::new(start, end)
    }
}

#[cfg(test)]
#[test]
fn addr_range_test() {
    use crate::sys_utility::block_servant::{VirtualAddr, VirtualRange};

    use super::block_servant::VirtualAddrCount;
    let vr = VirtualRange::with_size(VirtualAddr::new(10), 10000);
    let range = AddrRange::from_block_servant_range(BlockServantOffsetRange::new(
        BlockAddr { addr: 1 },
        vr,
    ));
    println!("{:?}", range);
}
