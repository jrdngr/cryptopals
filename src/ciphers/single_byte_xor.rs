pub fn single_byte_xor(bytes: &[u8], xor_byte: u8) -> Vec<u8> {
    bytes.iter().map(|byte| byte ^ xor_byte).collect()
}
