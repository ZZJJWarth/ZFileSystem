struct Test {
    a: u32,
    b: u32,
}

// impl AsRef<[u8]> for Test{

// }

#[cfg(test)]
#[test]
fn test_main() {
    use std::{fs::File, io::BufWriter};

    let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let aa = a.as_slice().as_ptr();
    let aaa = unsafe { aa as *const &[u8; 5] };
    let aa = unsafe { aa.offset(5) as *const &[u8; 5] };

    let f = File::options().write(true).open("../apiTest").unwrap();
    let br = BufWriter::with_capacity(5, f);
}
//0100 0011 1010 1001 1100 1100 0010 0010
