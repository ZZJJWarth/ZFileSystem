use std::fs::File;
use std::{fs,path};
use std::convert::AsRef;
use std::io::{BufReader, BufRead,BufWriter,Write};
use super::function::{u8_to_u32, u32_to_vec};
use super::addr::Addr;
use std::io::{Seek,SeekFrom};
#[derive(Debug)]
enum FindErr{
    Full,

}

static BLOCK_SIZE:u32=1024; //限制块大小为1k
static NON_OCCUPY_NUM:u32=1135201314;
static LAST_BLOCK:u32=1135211314;

#[derive(Debug)]
pub struct BitMap{
    block_num:u32,      //表示管理的块的多少
    //data_entry:Addr,
    bitmap_entry:Addr,   //bitmap数据区的入口
    reserved_block_num:u32,
    file_path:&'static str  //表示虚拟磁盘文件的地址
    
}



fn get_block_num(btype_num:u64)->u32{
    (btype_num/1024) as u32
}

fn get_bitmap_block_num(btype_num:u64)->u32{
    let block=get_block_num(btype_num);
    if (block%256==0){
        (block/256) as u32
    }else{
        (block/256+1) as u32
    }
}

fn get_block_entry(block_addr:u32)->u32{
    block_addr*1024
}



impl BitMap{
    fn new(virtual_disk_path:&'static str,super_block_len:u32)->BitMap{
        let f=fs::File::open(virtual_disk_path).unwrap();
        let meta=f.metadata().unwrap();
        let b_block=get_bitmap_block_num(meta.len())+super_block_len;     //这个是获得了超级块和bitmap需要的块个数
        let entry=super_block_len*BLOCK_SIZE;                            //获取了bitmap的入口位置
        let all_block=get_block_num(meta.len());               //文件总的块个数
        let bitmap=BitMap{
            block_num:all_block,
            //data_entry:0,
            bitmap_entry:Addr::new(entry),
            reserved_block_num:b_block,
            file_path:virtual_disk_path,
        };
        bitmap
    }

    fn set_non_occupied(&self,addr:&Addr)->Result<(),&str>{
        // if(addr.get_raw_num()>=self.block_num){
        //     return Err("you are trying to use address out of disk!");
        // }
        self.set_value(addr, NON_OCCUPY_NUM);
        Ok(())
    }

    fn check_non_occupied(&self,addr:&Addr)->Result<bool,&str>{
        // if(addr.get_raw_num()>=self.block_num){
        //     return Err("you are trying to use address out of disk!");
        // }
        let block_entry=self.bitmap_entry.get_raw_num()+(addr.get_raw_num()/BLOCK_SIZE)*4;
        let f=File::open(self.file_path).unwrap();
        let mut br=BufReader::with_capacity(4, f);
        
        br.seek(SeekFrom::Start(block_entry as u64));
        br.fill_buf();
        let content=br.buffer();
        let content=u8_to_u32(&Vec::from(content));
        if content==NON_OCCUPY_NUM{
            Ok(true)
        }else{
            Ok(false)
        }

    }

    fn set_value(&self,addr:&Addr,value:u32){
        let block_entry=self.bitmap_entry.get_raw_num()+(addr.get_raw_num()/BLOCK_SIZE)*4;    //获得即将标记的地址
        let occupy_u8=u32_to_vec(value);                                           //把OCCUPY_NUM转换成u8向量
        let f=File::options().write(true).open(self.file_path).unwrap();
        let mut bf=BufWriter::with_capacity(4, f);
        bf.seek(SeekFrom::Start(block_entry as u64));
        bf.write(&occupy_u8);
    }

    fn init(&self){
        let entry=self.bitmap_entry.get_raw_num();               //入口地址
        let all_block_num=&self.block_num;          //块总数，这里指102400
        let bitmap_end=(entry+self.reserved_block_num)*BLOCK_SIZE/4;   //bitmap管理区域的终点地址
        let occupy_end=entry+self.reserved_block_num;    //初始化中非空区的终点地址
        let bm_block_size=all_block_num*4;     //bitmap初始化要写的区域的总大小,这里指409600
        let mut i=entry;
        let f=File::options().write(true).open(self.file_path).unwrap();
        let mut bf=BufWriter::with_capacity(bm_block_size as usize, f);
        bf.seek(SeekFrom::Start(entry as u64));
        let mut list:Vec<u8>=vec![];
        while(i<bitmap_end){
            if(i<occupy_end){
                list.append(&mut u32_to_vec(i));
            }else{
                list.append(&mut u32_to_vec(NON_OCCUPY_NUM));
            }
            i=i+1;
        }
        bf.write(&list);
    }

    fn free_block(&self)->Result<u32,FindErr>{      //找到一个空闲块，并返回块号
        // let entry=Addr::new(self.bitmap_entry.get_raw_num());               //入口地址
        // let occupy_end=entry.get_raw_num()+self.reserved_block_num;       //非空区的终点地址
        // let bitmap_end=(entry+self.reserved_block_num)*BLOCK_SIZE/4;   //bitmap管理区域的终点地址
        // let mut i=occupy_end;
        // while(i<bitmap_end){
        //     println!("{}",i);
        //     if(self.check_non_occupied(&Addr::new(i*1024)).unwrap()){
        //         self.set_value(&Addr::new(i),LAST_BLOCK);
        //         break;
        //     }else{
        //         i=i+1;
        //     }

        // }
        // if(bitmap_end>=i){
        //     Err(FindErr::Full)
        // }else{
        //     Ok(i)
        // }
        Ok(12 as u32)
        
    }
    
}

#[cfg(test)]

#[test]
fn test1()->std::io::Result<()>{
    let f=File::open("../test")?;
    let me=f.metadata()?;
    let block_num=get_block_num(me.len());
    let bit_num=get_bitmap_block_num(me.len());
    assert_eq!(get_block_num(me.len()),102400);
    assert_eq!(bit_num,400);
    Ok(())
}

fn test2(){
    let bm=BitMap::new("../apiTest", 0);
    let addr=Addr::new(1024);
    bm.set_non_occupied(&addr);
    assert!(bm.check_non_occupied(&addr).unwrap());
}

#[test]
fn test3(){
    let bm=BitMap::new("../test",0);
    assert!(bm.check_non_occupied(&Addr::new(409600)).unwrap());
    assert!(!bm.check_non_occupied(&Addr::new(409599)).unwrap());

}

#[test]
fn test4(){
    // let bm=BitMap::new("./test1",0);
    let f=File::open("../test1").unwrap();
    let mut br=BufReader::with_capacity(4, f);
    br.seek_relative(16);
    br.fill_buf();
    let con=br.buffer();
    let a=u8_to_u32(&Vec::from(con));
    println!("{}",a);
}   

#[test]
fn test5(){
    let bm=BitMap::new("../test",0);
    // bm.init();
    let a=bm.free_block();
    match a{
        Ok(block_num)=>{println!("{}",block_num)},
        Err(e)=>{println!("{:?}",e)}
    }   
}