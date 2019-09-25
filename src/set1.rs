use crate::conversion::base64::hex_to_base64;
use crate::conversion::hex::{hex_string_to_bytes, bytes_to_hex_string};
use crate::byte_operations::{bytes_xor, bytes_xor_single_byte};

// Convert hex to base64
// input: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
// output: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
//
// Rule: Always operate on raw bytes, never on encoded strings. Only use hex and base64 for
// pretty-printing.
#[test]
pub fn challenge_1() {
    let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let test_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = hex_to_base64(test_string);

    assert_eq!(result, expected_output);
}


// Write a function that takes two equal-length buffers and produces their XOR combination.
// If your function works properly, then when you feed it the string:
// 1c0111001f010100061a024b53535009181c
// ... after hex decoding, and when XOR'd against:
// 686974207468652062756c6c277320657965
// ... should produce:
// 746865206b696420646f6e277420706c6179
#[test]
pub fn challenge_2() {
    let expected_output = "746865206b696420646f6e277420706c6179";

    let bytes1 = hex_string_to_bytes("1c0111001f010100061a024b53535009181c");
    let bytes2 = hex_string_to_bytes("686974207468652062756c6c277320657965");
    
    let result_bytes = bytes_xor(&bytes1, &bytes2);
    let result = bytes_to_hex_string(&result_bytes);
     
    assert_eq!(result, expected_output);
}

//  The hex encoded string:
// 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
// ... has been XOR'd against a single character. Find the key, decrypt the message.
// You can do this by hand. But don't: write code to do it for you.
// How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score. 
#[test]
pub fn challenge_3() {
    use std::collections::HashMap;

    let bytes = hex_string_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    
    let mut result: HashMap<String, usize> = HashMap::new();

    // UTF-8 error beyond 127. Ignoring for now.
    for xor_byte in 0..127 {
        let mut character_counts: HashMap<u8, usize> = HashMap::new();

        let result_bytes = bytes_xor_single_byte(&bytes, xor_byte);
        let result_string = std::str::from_utf8(&result_bytes).unwrap().to_string();

        for byte in result_bytes {
            let count = character_counts.entry(byte).or_insert(0);
            *count += 1;
        }

        fn is_letter(character: u8) -> bool {
            (character >= 65 && character <= 90) || (character >= 97 && character <=122)
        }

        let score: usize = character_counts
            .into_iter()
            .filter(|(character, _)| is_letter(*character))
            .map(|(character, count)| character as usize * count)
            .sum();

        result.insert(result_string, score);
    }

    let message = result.into_iter().max_by_key(|(_, score)| score.clone()).unwrap().0;

    assert_eq!(message, "Cooking MC's like a pound of bacon");
}
