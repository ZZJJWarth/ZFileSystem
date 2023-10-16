use std::{io::{BufReader, BufRead, Seek}, fs::File, mem::transmute};

use super::{file_writer::IoOption, config::FILE_PATH, addr::BlockAddr, bitmap_servant::BlockOffset};
#[derive(Debug)]
pub struct FileReader{
    br:BufReader<File>,
}

impl FileReader{
    pub fn new(option:IoOption)->FileReader{
        
        let f=File::open(FILE_PATH).unwrap();
        let br=match option {
            IoOption::Bitmap=>{
                BufReader::with_capacity(4, f)
            }
            IoOption::Other(buf)=>{
                BufReader::with_capacity(buf as usize, f)
            }
        };
        FileReader{
            br
        }
    }

    pub fn bitmap_read(&mut self,offset:BlockOffset)->BlockAddr{
        let offset=offset.addr_offset();
        // println!("offset={:?}",offset);
        self.br.seek(std::io::SeekFrom::Start(offset.get_raw_num() as u64));
        self.br.fill_buf();
        let a=self.br.buffer().as_ptr();
        // println!("a={:?}",a);
        let num:BlockAddr=unsafe {
            *(a as *const BlockAddr)
        };
        num
    }
}

#[cfg(test)]

// #[test]
fn test1(){
    let mut fr=FileReader::new(IoOption::Bitmap);
    let addr=fr.bitmap_read(BlockOffset::new(BlockAddr { addr: 500 }));
    println!("{:?}",addr);
}