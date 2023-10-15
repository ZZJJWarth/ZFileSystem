use super::addr::{BlockAddr,BlockRange};
use super::config::FILE_PATH;


struct BlockBitmap {
    
    bitmap_entry:BlockAddr,
    block_num:u32,
    reserved_block_num:u32,
}

impl BlockBitmap{
    fn new(bitmap_entry:BlockAddr,block_num:u32,reserved_block_num:u32)->Self{
        Self{
            bitmap_entry,
            block_num,
            reserved_block_num,
        }
    }

    fn init(&self){
        let range=Self::get_blockrange_of_file();
        for i in range.iter(){
            
        }
    }

    fn get_blockrange_of_file()->BlockRange{
        let start=BlockAddr::new(0);
        let end=Self::file_max_block();
        BlockRange::new(start, end)
    }
///找到文件的最大block
    fn file_max_block() -> BlockAddr {  //向下调用接口
        todo!()
    }
///给定一个blockAddr，将对应的Bitmap修改成未占用
    fn set_non_occupied(&self,block_addr:BlockAddr){       
        todo!()
    }
///给定一个blockAddr,将对应的Bitmap修改成指定的BlockAddr
    fn set_value(&self,block_addr:BlockAddr,value:BlockAddr){
        todo!()
    }

}

