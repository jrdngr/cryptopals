use crate::decode;
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
    let bytes =
        hex_string_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let (message, _) = decode::single_byte_xor(&bytes);

    assert_eq!(message, "Cooking MC's like a pound of bacon");
}

// One of the 60-character strings in the file 4.txt has been encrypted by single-character XOR.
// Find it.
// (Your code from #3 should help.) 
#[test]
fn challenge_4() {
    use std::collections::HashMap;
    use std::io::{BufRead, BufReader};
    use std::fs::File;

    let file = File::open("files/4.txt").unwrap();
    let reader = BufReader::new(file);

    let mut results = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let bytes = hex_string_to_bytes(&line);

        let (message, score) = decode::single_byte_xor(&bytes);
        results.insert(score, message);
    }


    let (score, message) = results
        .into_iter()
        .max_by_key(|(_, score)| score.clone())
        .unwrap();

    dbg!(message);
    dbg!(score);
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
pub fn challenge_5() {
    use crate::ciphers::repeating_key_xor;

    let input_line_1 = "Burning 'em, if you ain't quick and nimble";
    let input_line_2 = "I go crazy when I hear a cymbal";
    let key = "ICE";

    let key_bytes: Vec<u8> = key.bytes().collect();

    let result_line_1 = repeating_key_xor(input_line_1.as_bytes(), &key_bytes);
    let result_line_1 = bytes_to_hex_string(&result_line_1);
    
    let result_line_2 = repeating_key_xor(input_line_2.as_bytes(), &key_bytes);
    let result_line_2 = bytes_to_hex_string(&result_line_2);

    assert_eq!(result_line_1, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272");
    assert_eq!(result_line_2, "a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
}
