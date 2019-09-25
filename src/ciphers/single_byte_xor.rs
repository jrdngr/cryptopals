use crate::byte_operations::bytes_xor;

pub fn single_byte_xor(bytes: &[u8], xor_byte: u8) -> Vec<u8> {
    let xor_bytes = vec![xor_byte; bytes.len()];
    bytes_xor(&bytes, &xor_bytes)
}