pub fn bytes_xor(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    bytes1.iter()
        .zip(bytes2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect()
}

pub fn bytes_xor_single_byte(bytes: &[u8], xor_byte: u8) -> Vec<u8> {
    let xor_bytes = vec![xor_byte; bytes.len()];
    bytes_xor(&bytes, &xor_bytes)
}