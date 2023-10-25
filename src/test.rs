struct Test {
    a: u32,
    b: u32,
}

// impl AsRef<[u8]> for Test{

// }

#[cfg(test)]
// #[test]
fn test_main() {
    use std::io::stdin;

    let mut str=String::new();
    let si=stdin();
    si.read_line(&mut str).unwrap();
    println!("{}",str);
}
//0100 0011 1010 1001 1100 1100 0010 0010
