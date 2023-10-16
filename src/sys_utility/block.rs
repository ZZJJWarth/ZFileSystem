use std::fs::File;

use super::{addr::BlockAddr, bitmap_servant::BlockOffset, config::FILE_PATH, file_writer::FileWriter};

enum BlockOption {
    Read,
    Write,
}


struct Block{
    data:Vec<u8>,
    dirty:bool,
}



impl Block{
    pub fn new(b_num:BlockAddr,options:BlockOption){
        let f=match options{
            BlockOption::Read=>{
                File::open(FILE_PATH).unwrap()
            },
            BlockOption::Write=>{
                File::options().write(true).open(FILE_PATH).unwrap()
            }
        };

    }
}