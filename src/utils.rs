pub fn print_bits(bytes: &[u8]) {
    for byte in bytes {
        print!("{:08b}, ", byte);
    }
    println!();
}