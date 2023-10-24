use core::panic;

use super::super::{
    addr::addr::{BlockAddr, BlockCount, BlockRange},
    bitmap::bitmap_servant::BitmapServant,
    config::config::{END_NUM, FILE_PATH, NON_OCCUPY_NUM},
};

#[derive(Debug)]
pub struct BlockBitmap {
    servant: BitmapServant,
    block_num: u32,
    reserved_block_num: u32,
}

impl BlockBitmap {
    pub fn new(bitmap_entry: BlockAddr, block_num: u32, reserved_block_num: u32) -> Self {
        Self {
            servant: BitmapServant::new(bitmap_entry),
            block_num,
            reserved_block_num,
        }
    }

    ///寻找一个空闲块并返回，TODO：可以完善错误机制
    pub fn get_free_block(&mut self) -> Result<BlockAddr, ()> {
        let range = self.get_blockrange_of_file();
        for i in range.iter() {
            if (self.check_block_empty(i)) {
                self.servant.set_value(i, END_NUM); //返回的空闲块认为是最后一块
                return Ok(i);
            }
        }
        Err(())
    }
    ///格式化整个空间
    pub fn init(&mut self) {
        let range = self.get_blockrange_of_file();
        let mut count = BlockCount::new(self.reserved_block_num);
        for i in range.iter() {
            if (count.reduce()) {
                self.servant.set_value(i, i);
            } else {
                self.servant.set_non_occupied(i);
            }
        }
    }

    pub fn set_value(&mut self, block: BlockAddr, value: BlockAddr) {
        self.servant.set_value(block, value);
    }

    fn get_blockrange_of_file(&self) -> BlockRange {
        let start = BlockAddr::new(0);
        let end = self.servant.file_max_block();
        BlockRange::new(start, end)
    }

    pub fn check_block_empty(&mut self, block: BlockAddr) -> bool {
        self.servant.check_block_empty(block)
    }
    ///使得一个块变为empty
    pub fn set_empty_block(&mut self, block: BlockAddr) {
        self.servant.set_non_occupied(block);
    }
    ///获取位图中块的信息，需要注意，位图中存储的东西其实就是BlockAddr，这个东西相当于一个链表
    pub fn get_content(&mut self, block: BlockAddr) -> BlockAddr {
        if (block == NON_OCCUPY_NUM || block == END_NUM) {
            panic!("尝试查找空块或结束块！");
        }
        self.servant.read_a_block(block)
    }

    pub fn find_final_block(&mut self, ba: BlockAddr) -> Result<BlockAddr, ()> {
        let mut ba = ba;
        let mut nba = self.get_content(ba);
        // self.test_block(ba);
        // println!("nba:{:?}",nba);
        let mut count = 10;
        while (nba != END_NUM && nba != NON_OCCUPY_NUM) {
            // count-=1;
            // if(count==0){
            //     break;
            // }
            // println!("nba:{:?}",nba);
            ba = nba;
            nba = self.get_content(ba);
        }
        if ba == NON_OCCUPY_NUM {
            Err(())
        } else {
            Ok(ba)
        }
    }

    pub fn reduce_a_block(&mut self, block: BlockAddr) {
        if (block == NON_OCCUPY_NUM) {
            panic!("减少空块函数需要一个根块，但是接收到了空块");
        }
        let mut node = block;
        let mut next = self.get_content(block);
        if (next == END_NUM) {
            self.set_empty_block(node);
        }

        while (self.get_content(next) != END_NUM) {
            node = next;
            next = self.get_content(next);
        }
        self.set_empty_block(next);
        self.set_value(node, END_NUM);
    }

    pub fn add_block(&mut self, block: BlockAddr) -> BlockAddr {
        let b = self.get_free_block().unwrap();

        self.set_value(block, b);
        b
    }

    pub fn test_block(&mut self, block: BlockAddr) {
        let mut count = 10;
        let mut block = block;
        while (block != NON_OCCUPY_NUM && block != END_NUM && count > 0) {
            count = count - 1;
            println!("test {}:{:?}", count, block);
            block = self.get_content(block);
        }
    }
}

#[cfg(test)]
// #[test]
fn test1() {
    let mut a = BlockBitmap::new(BlockAddr::new(0), 10, 1);
    println!("{:?}", a.get_content(BlockAddr::new(1)));
}

// #[test]
fn test2() {
    //todo:这个bitmap是测试使用的，真正运行的时候应该是用应该static的bitmap
    let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
    bit_map.init();
    // println!("content a:{:?}", bit_map.get_content(BlockAddr { addr: 2 }));
    bit_map.get_free_block();
}
