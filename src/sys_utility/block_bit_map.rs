use super::addr::{BlockAddr,BlockRange,BlockCount};
use super::bitmap_servant::BitmapServant;
use super::config::{FILE_PATH, END_NUM};

#[derive(Debug)]
pub struct BlockBitmap {
    servant:BitmapServant,
    block_num:u32,
    reserved_block_num:u32,
}

impl BlockBitmap{
    pub fn new(bitmap_entry:BlockAddr,block_num:u32,reserved_block_num:u32)->Self{
        
        Self{
            servant:BitmapServant::new(bitmap_entry),
            block_num,
            reserved_block_num,
        }
    }

///寻找一个空闲块并返回，TODO：可以完善错误机制
    pub fn get_free_block(&mut self)->Result<BlockAddr,()>{
        let range =self.get_blockrange_of_file();
        for i in range.iter(){
            if(self.check_block_empty(i)){
                self.servant.set_value(i, END_NUM); //返回的空闲块认为是最后一块
                return Ok(i);
            }
        }   
        Err(())
    }
///格式化整个空间
    pub fn init(&mut self){
        let range=self.get_blockrange_of_file();
        let mut count=BlockCount::new(self.reserved_block_num);
        for i in range.iter(){
            if(count.reduce()){
                self.servant.set_value(i, i);
            }else{
                self.servant.set_non_occupied(i);
            }
        }
    }

    fn get_blockrange_of_file(&self)->BlockRange{
        let start=BlockAddr::new(0);
        let end=self.servant.file_max_block();
        BlockRange::new(start, end)
    }

    fn check_block_empty(&mut self,block:BlockAddr)->bool{
        self.servant.check_block_empty(block)
    }
///使得一个块变为empty
    pub fn set_empty_block(&mut self,block:BlockAddr){
        self.servant.set_non_occupied(block);
    }
///获取位图中块的信息，需要注意，位图中存储的东西其实就是BlockAddr，这个东西相当于一个链表
    pub fn get_content(&mut self,block:BlockAddr)->BlockAddr{
        self.servant.read_a_block(block)
    }


}


#[cfg(test)]

#[test]
fn test1(){
    let mut a=BlockBitmap::new(BlockAddr::new(0), 10, 1);
    println!("{:?}",a.get_content(BlockAddr ::new(1)));
}
