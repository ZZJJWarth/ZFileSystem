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
        
    }

    fn get_blockrange_of_file()->BlockRange{
        let start=BlockAddr::new(0);
        let end=Self::file_max_block();
        BlockRange::new(start, end)
    }

    fn file_max_block() -> BlockAddr {  //向下调用接口
        todo!()
    }

    fn set_non_occupied(&self,block_addr:BlockAddr){
        todo!()
    }

    fn set_value(&self,block_addr:BlockAddr,value:BlockAddr){
        todo!()
    }

}

