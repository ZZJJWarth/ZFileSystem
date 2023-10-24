use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    mem::transmute,
};

use super::super::{
    addr::addr::{Addr, BlockAddr},
    bitmap::bitmap_servant::BlockOffset,
    block::block_servant::{BlockServantOffsetRange, DataPack},
    config::config::{BLOCK_SIZE, FILE_PATH},
};

pub enum IoOption {
    Bitmap,
    Other(u32),
}
#[derive(Debug)]
pub struct FileWriter {
    bf: BufWriter<File>,
    br: BufReader<File>,
}

impl FileWriter {
    pub fn new(opt: IoOption) -> FileWriter {
        let f = File::options().write(true).open(FILE_PATH).unwrap();
        match opt {
            IoOption::Bitmap => {
                let mut bf = BufWriter::with_capacity(4 as usize, f);
                let f = File::open(FILE_PATH).unwrap();
                FileWriter {
                    bf,
                    br: BufReader::with_capacity(4 as usize, f),
                }
            }
            IoOption::Other(cap) => {
                let mut bf = BufWriter::with_capacity(BLOCK_SIZE as usize, f);
                let f = File::open(FILE_PATH).unwrap();
                let br = BufReader::with_capacity(BLOCK_SIZE as usize, f);
                FileWriter { bf, br }
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
        let ptr = dp.write_in;
        let ar = AddrRange::from_block_servant_range(br);
        let entry = br.get_block_entry().into_addr().get_raw_num();
        let offset = AddrRange::from_block_servant_range(br);
        self.br.seek(SeekFrom::Start(entry as u64)).unwrap();
        self.br.fill_buf();
        let mut block: [u8; BLOCK_SIZE as usize] = [0; BLOCK_SIZE as usize];
        let temp = self.br.buffer();
        for i in 0..BLOCK_SIZE as usize {
            block[i] = temp[i];
        }
        let start = ar.start.get_raw_num() % BLOCK_SIZE;
        let mut end = ar.end.get_raw_num() % BLOCK_SIZE;
        if (end == 0) {
            end = BLOCK_SIZE;
        }
        let mut count = dp.index;
        for i in start..end {
            block[i as usize] = ptr[count];

            count = count + 1;
        }
        self.bf.seek(SeekFrom::Start(entry as u64));
        // println!("写入地址为：{}",entry);
        // println!("写入数据为：{:?}",ptr);
        self.bf.write(&block);
        Ok(())
    }

    // pub fn read_block(&mut self)
}

#[cfg(test)]

// #[test]
fn test1() {
    use crate::sys_utility::block::file_reader::FileReader;

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
pub struct AddrRange {
    pub start: Addr,
    pub end: Addr,
    pub len: u32,
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
// #[test]
fn addr_range_test() {
    use crate::sys_utility::block::block_servant::{VirtualAddr, VirtualRange};

    use super::block_servant::VirtualAddrCount;
    let vr = VirtualRange::with_size(VirtualAddr::new(10), 10000);
    let range = AddrRange::from_block_servant_range(BlockServantOffsetRange::new(
        BlockAddr { addr: 1 },
        vr,
    ));
    println!("{:?}", range);
}
