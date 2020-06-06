use std::collections::HashSet;

use crate::ciphers;

pub struct SingleByteXorResult {
    pub message: String,
    pub score: usize,
    pub xor_byte: u8,
}

pub fn single_byte_xor<B: AsRef<[u8]>>(bytes: B) -> SingleByteXorResult {
    let mut result: Vec<SingleByteXorResult> = Vec::new();

    let most_common_letters: HashSet<char> = [
        'E', 'T', 'A', 'O', 'I', 'N', 'S', 'H', 'R', 'D', 'L', 'U', 'e', 't', 'a', 'o', 'i', 'n',
        's', 'h', 'r', 'd', 'l', 'u', ' ',
    ]
    .iter()
    .copied()
    .collect();

    for xor_byte in 0..127 {
        let result_bytes = ciphers::single_byte_xor(bytes.as_ref(), xor_byte);
        let message = String::from_utf8_lossy(&result_bytes).into_owned();

        let score: usize = result_bytes
            .into_iter()
            .filter(|byte| most_common_letters.contains(&(*byte as char)))
            .count();

        result.push(SingleByteXorResult { message, score, xor_byte });
    }

    result
        .into_iter()
        .max_by_key(|SingleByteXorResult { score, ..}| score.clone())
        .unwrap()
}

pub fn repeating_byte_xor<B: AsRef<[u8]>>(bytes: B) -> Vec<String> {
    let bytes = Vec::from(bytes.as_ref());

    let mut result = Vec::new();

    let key_sizes = rank_key_sizes(&bytes, 40);
    for key_size in &key_sizes[0..3] {
        let blocks = transpose_by_key_size(&bytes, *key_size);
        
        let mut key = Vec::new();

        for block in blocks {
            let single_byte_xor_result = single_byte_xor(&block);
            key.push(single_byte_xor_result.xor_byte);
        }

        let message = ciphers::repeating_key_xor(&bytes, &key);
        result.push(String::from_utf8_lossy(&message).to_string());
    }

    result
}

fn rank_key_sizes(bytes: &[u8], max_key_size: usize) -> Vec<usize> {
    use crate::math::metrics::hamming_distance;

    let max_key_size = max_key_size.min(bytes.len() / 4);

    let mut key_size_scores = Vec::new();

    for key_size in 2..max_key_size {
        let mut blocks = Vec::new();

        for i in 0..4 {
            blocks.push(&bytes[(i*key_size)..((i+1) * key_size)]);
        }

        let mut distances = Vec::new();
        for i in 0..4 {
            for j in (i+1)..3 {
                let first_block = &blocks[i];
                let second_block = &blocks[j];
        
                let distance = hamming_distance(first_block, second_block);
                distances.push(distance);
            }
        }

        let sum: usize = distances.iter().sum();
        let score =  sum / distances.len();

        key_size_scores.push((score, key_size));
    }

    key_size_scores.sort_by_key(|(score, _)| *score);

    key_size_scores.into_iter().map(|(_, key_size)| key_size).collect()
}

fn transpose_by_key_size(bytes: &[u8], key_size: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    for _ in 0..key_size {
        result.push(Vec::new());
    }

    for (index, byte) in bytes.iter().cloned().enumerate() {
        let bucket = index % key_size;
        result[bucket].push(byte);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversion::hex::hex_string_to_bytes;

    #[test]
    fn test_guess_key_size() {
        let encrypted = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        // let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        // let key = "ICE";

        let result = rank_key_sizes(&hex_string_to_bytes(encrypted), 10);

        let mut is_answer_in_top_3 = false;

        for key_size in result {
            if key_size == 3 {
                is_answer_in_top_3 = true;
                break;
            }
        }
    
        assert!(is_answer_in_top_3);
    }
}
