use crate::byte_operations::bytes_xor;
use crate::conversion::base64::hex_to_base64;
use crate::conversion::hex::{bytes_to_hex_string, hex_string_to_bytes};

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
    use crate::ciphers::single_byte_xor;
    use std::collections::HashMap;

    let bytes =
        hex_string_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let mut result: HashMap<String, usize> = HashMap::new();

    // UTF-8 error beyond 127. Ignoring for now.
    for xor_byte in 0..127 {
        let mut character_counts: HashMap<u8, usize> = HashMap::new();

        let result_bytes = single_byte_xor(&bytes, xor_byte);
        let result_string = std::str::from_utf8(&result_bytes).unwrap().to_string();

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

    let message = result
        .into_iter()
        .max_by_key(|(_, score)| score.clone())
        .unwrap()
        .0;

    assert_eq!(message, "Cooking MC's like a pound of bacon");
}

// Implement repeating-key XOR
// Here is the opening stanza of an important work of the English language:
//
// Burning 'em, if you ain't quick and nimble
// I go crazy when I hear a cymbal

// Encrypt it, under the key "ICE", using repeating-key XOR.
// In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I,
// the next C, the next E, then I again for the 4th byte, and so on.
// It should come out to:

// 0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
// a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f

// Encrypt a bunch of stuff using your repeating-key XOR function. Encrypt your mail. Encrypt your password file. Your .sig file.
// Get a feel for it. I promise, we aren't wasting your time with this.
#[test]
pub fn challenge_4() {
    use crate::ciphers::repeating_key_xor;

    let input_line_1 = "Burning 'em, if you ain't quick and nimble";
    let input_line_2 = "I go crazy when I hear a cymbal";
    let key = "ICE";

    let expected_output_line_1 = hex_string_to_bytes(
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272",
    );
    let expected_output_line_2 = hex_string_to_bytes(
        "a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
    );

    let key_bytes = hex_string_to_bytes(key);

    let result_line_1 = repeating_key_xor(input_line_1.as_bytes(), &key_bytes);
    let result_line_2 = repeating_key_xor(input_line_2.as_bytes(), &key_bytes);

    assert_eq!(result_line_1, expected_output_line_1);
    assert_eq!(result_line_2, expected_output_line_2);
}
