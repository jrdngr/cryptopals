pub fn print_bits(bytes: &[u8]) {
    for byte in bytes {
        print!("{:08b}, ", byte);
    }
    println!();
}

pub fn print_6_bits(bytes: &[u8]) {
    for byte in bytes {
        print!("{:06b}, ", byte);
    }
    println!();
}
