// Encrypt it, under the key "ICE", using repeating-key XOR.
// In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I,
// the next C, the next E, then I again for the 4th byte, and so on.
pub fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut xor_bytes = key.iter().cycle();
    let mut next_xor = || xor_bytes.next().expect("This iterator never ends");

    bytes.iter().map(|byte| byte ^ next_xor()).collect()
}

// 0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
