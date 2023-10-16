use super::addr::{BlockAddr,BlockRange,BlockCount};
use super::bitmap_servant::BitmapServant;
use super::config::FILE_PATH;

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

    fn check_block_empty(&mut self,block:BlockAddr){
        self.servant.check_block_empty(block);
    }

}


#[cfg(test)]

// #[test]
fn test1(){
    let mut a=BlockBitmap::new(BlockAddr::new(0), 10, 1);
    a.init();
}
