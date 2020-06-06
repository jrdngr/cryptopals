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

pub fn repeating_byte_xor<B: AsRef<[u8]>>(bytes: B) -> (String, usize) {
    let bytes = Vec::from(bytes.as_ref());

    let key_size = guess_key_size(&bytes);

    todo!()
}

type Score = usize;
type KeySize = usize;

fn guess_key_size(bytes: &[u8]) -> Vec<(Score, KeySize)> {
    use crate::math::metrics::hamming_distance;

    let length = bytes.len();
    let max_key_size = 40.min(length / 4);

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

    key_size_scores
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

        let result = guess_key_size(&hex_string_to_bytes(encrypted));

        let mut is_answer_in_top_3 = false;

        for (_, key_size) in result {
            if key_size == 3 {
                is_answer_in_top_3 = true;
                break;
            }
        }
    
        assert!(is_answer_in_top_3);
    }
}
