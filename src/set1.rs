use crate::conversion::base64::hex_to_base64;
use crate::conversion::hex::{hex_string_to_bytes, bytes_to_hex_string};
use crate::byte_operations::bytes_xor;

// Convert hex to base64
// input: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
// output: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
//
// Rule: Always operate on raw bytes, never on encoded strings. Only use hex and base64 for
// pretty-printing.
#[test]
pub fn challenge1() {
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
