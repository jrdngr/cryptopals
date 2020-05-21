// Encrypt it, under the key "ICE", using repeating-key XOR.
// In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I,
// the next C, the next E, then I again for the 4th byte, and so on.
pub fn repeating_key_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    use crate::byte_operations::bytes_xor;

    Vec::new()
}
