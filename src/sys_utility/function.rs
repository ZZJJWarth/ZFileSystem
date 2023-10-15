pub fn u32_to_vec(num: u32) -> Vec<u8> {
    // {
    //     let mut num = num;
    //     let mut list: Vec<u8> = vec![];
    //     let bitmask: u32 = 255;
    //     for i in 0..4 {
    //         let temp = (num & bitmask) as u8;
    //         num = num >> 8;
    //         list.push(temp);
    //     }
    //     list
    // }

    let ptr = &num as *const u32 as *const u8;
    let x = unsafe { core::slice::from_raw_parts(ptr, core::mem::size_of::<u32>()).to_vec() };
    return x;
}

pub fn u8_to_u32(data: &Vec<u8>) -> u32 {
    assert!(data.len() == 4);
    // {
    //     let bitmask: u32 = 0b11111111_11111111_11111111_11111111;
    //     let mut ans: u32 = 0;

    //     for i in data.iter().rev() {
    //         let n = *i as u32;
    //         ans = ans << 8;
    //         ans = ans | (n & bitmask);
    //     }

    //     ans
    // }

    {
        let ptr = data.as_slice().as_ptr() as *const u32;
        unsafe { *ptr }
    }
}

#[cfg(test)]
#[test]
fn test1() {
    // let mut a:u32=1135201314;
    // let A=a;
    // let mut list:Vec<u8>=vec![];
    // let bitmask:u32=255;
    // for i in 0..4{
    //     let temp=(a&bitmask) as u8;
    //     a=a>>8;
    //     list.push(temp);
    // }
    // let bitmask:u32=0b11111111_11111111_11111111_11111111;
    // let mut after_a:u32=0;

    // for i in list.iter().rev() {
    //     let n=*i as u32;
    //     after_a=after_a<<8;
    //     after_a=after_a|(n&bitmask);

    // }
    let a: u32 = 12648614;
    assert_eq!(a, u8_to_u32(&u32_to_vec(a)));
}
