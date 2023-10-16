use std::{fs::File, io::{BufWriter, Seek, SeekFrom, Write}, mem::transmute};

use super::{bitmap_servant::BlockOffset, config::FILE_PATH, addr::BlockAddr};

pub enum WriterOption{
    Bitmap,
    Other(u32),
}
#[derive(Debug)]
pub struct FileWriter{
    bf:BufWriter<File>
}

impl FileWriter{
    pub fn new(opt:WriterOption)->FileWriter{
        let f=File::options().write(true).open(FILE_PATH).unwrap();
        match opt{
            WriterOption::Bitmap=>{
                let mut bf=BufWriter::with_capacity(4 as usize, f);
                FileWriter {
                    bf
                }
            }
            WriterOption::Other(cap)=>{
                let mut bf=BufWriter::with_capacity(cap as usize, f);
                FileWriter {
                    bf
                }
            }
        }
        
    }

    pub fn bitmap_write(&mut self,b_off:BlockOffset,value:BlockAddr){
        let addr=b_off.addr_offset();
        self.bf.seek(SeekFrom::Start(addr.get_raw_num() as u64));
        let buf=unsafe {
            transmute::<BlockAddr,[u8;4]>(value)
        };
        self.bf.write(&buf);
    }

    pub fn get_file_len(&self)->u64{
        let f=File::open(FILE_PATH).unwrap();
        let mt=f.metadata().unwrap();
        mt.len()
    }
}

// #[cfg(test)]

// #[test]
// fn test1(){
//     let f=FileWriter::new(WriterOption::Bitmap);
// }