use super::addr::{BlockAddr, BlockCount, BlockRange};
use super::bitmap_servant::BitmapServant;
use super::config::{END_NUM, FILE_PATH};

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
                // println!("{:?}",i);
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

    fn check_block_empty(&mut self, block: BlockAddr) -> bool {
        self.servant.check_block_empty(block)
    }
    ///使得一个块变为empty
    pub fn set_empty_block(&mut self, block: BlockAddr) {
        self.servant.set_non_occupied(block);
    }
    ///获取位图中块的信息，需要注意，位图中存储的东西其实就是BlockAddr，这个东西相当于一个链表
    pub fn get_content(&mut self, block: BlockAddr) -> BlockAddr {
        self.servant.read_a_block(block)
    }
}

#[cfg(test)]
#[test]
fn test1() {
    let mut a = BlockBitmap::new(BlockAddr::new(0), 10, 1);
    println!("{:?}", a.get_content(BlockAddr::new(1)));
}

// #[test]
fn test2() {
    let mut bit_map = BlockBitmap::new(BlockAddr { addr: 1 }, 256, 2); //测试用
    let a = bit_map.get_free_block().unwrap();
    let b = bit_map.get_free_block().unwrap();
    let c = bit_map.get_free_block().unwrap();
    bit_map.set_value(a, c);
    println!("a:{:?}", a);
    println!("b:{:?}", b);
    println!("c:{:?}", c);
    println!("content a:{:?}", bit_map.get_content(a));
    println!("content b:{:?}", bit_map.get_content(b));
    println!("content c:{:?}", bit_map.get_content(c));
}
