struct Test{
    a:u32,
    b:u32,
}

// impl AsRef<[u8]> for Test{

// }

#[cfg(test)]


fn test_main(){
    
    let mut a:u32=1135201314;
    let mut list:Vec<u8>=vec![];
    let bitmask:u32=255;
    for i in 0..4{
        let temp=(a&bitmask) as u8;
        a=a>>8;      
        list.push(temp);
    }
    let bitmask:u32=0b11111111_11111111_11111111_11111111;
    let mut after_a:u32=0;
    
    for i in list.iter().rev() {
        let n=*i as u32;
        after_a=after_a<<8;
        after_a=after_a|(n&bitmask);
        
    }

}
//0100 0011 1010 1001 1100 1100 0010 0010