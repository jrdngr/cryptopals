use std::collections::HashMap;

use crate::ciphers::single_byte_xor as encode_single_byte_xor;

pub fn single_byte_xor<B: AsRef<[u8]>>(bytes: B) -> (String, usize) {
    let mut result: HashMap<String, usize> = HashMap::new();

    for xor_byte in 0..255 {
        let mut character_counts: HashMap<u8, usize> = HashMap::new();

        let result_bytes = encode_single_byte_xor(bytes.as_ref(), xor_byte);
        let result_string = String::from_utf8_lossy(&result_bytes).into_owned();

        for byte in result_bytes {
            let count = character_counts.entry(byte).or_insert(0);
            *count += 1;
        }

        let score: usize = character_counts
            .into_iter()
            .filter(|(character, _)| {
                (*character >= 65 && *character <= 90) || (*character >= 97 && *character <= 122)
            })
            .map(|(character, count)| character as usize * count)
            .sum();

        result.insert(result_string, score);
    }

    let (message, score) = result
        .into_iter()
        .max_by_key(|(_, score)| score.clone())
        .unwrap();

    (message, score)
}
