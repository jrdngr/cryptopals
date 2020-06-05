use std::collections::HashSet;

use crate::ciphers::single_byte_xor as encode_single_byte_xor;

pub fn single_byte_xor<B: AsRef<[u8]>>(bytes: B) -> (String, usize) {
    let mut result: Vec<(String, usize)> = Vec::new();

    let most_common_letters: HashSet<char> = [
        'E', 'T', 'A', 'O', 'I', 'N', 'S', 'H', 'R', 'D', 'L', 'U', 'e', 't', 'a', 'o', 'i', 'n',
        's', 'h', 'r', 'd', 'l', 'u', ' ',
    ]
    .iter()
    .copied()
    .collect();

    for xor_byte in 0..127 {
        let result_bytes = encode_single_byte_xor(bytes.as_ref(), xor_byte);
        let result_string = String::from_utf8_lossy(&result_bytes).into_owned();

        let score: usize = result_bytes
            .into_iter()
            .filter(|byte| most_common_letters.contains(&(*byte as char)))
            .count();

        result.push((result_string, score));
    }

    let (message, score) = result
        .into_iter()
        .max_by_key(|(_, score)| score.clone())
        .unwrap();

    (message, score)
}

pub fn repeating_byte_xor<B: AsRef<[u8]>>(_bytes: B) -> (String, usize) {
    todo!()
}
