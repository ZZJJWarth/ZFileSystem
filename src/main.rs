#![allow(unused)]

use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};

mod sys_utility;
mod test;

fn main() -> std::io::Result<()> {
    // let a:u8=123;
    // fs::write("../apiTest",&a);
    // let f=File::open("../apiTest")?;
    // let mut bf=BufReader::with_capacity(10,f);
    // bf.seek_relative(5);
    // bf.fill_buf();
    // let mut f=File::options().write(true).open("/home/warth/projectZone/myFS/apiTest")?;
    // let mut bf=BufWriter::with_capacity(3, &f);
    // bf.seek(SeekFrom::Start(2));
    // let list="123456789";
    // println!("{}",bf.write(list).unwrap());
    // let a=bf.into_inner().unwrap();

    // println!("{:?}",list);
    use std::io::stdin;

    let mut str = String::new();
    let si = stdin();
    si.read_line(&mut str).unwrap();
    println!("{}", str);
    Ok(())
}
