use std::io::{BufReader, Read};

use crate::conversion::hex::hex_string_to_bytes;

#[rustfmt::skip]
const BASE_64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 
    'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', 
    '8', '9', '+', '/',
];

pub fn string_to_base64(string: &str) -> String {
    let bytes = string.bytes().collect::<Vec<u8>>();
    bytes_to_base64(&bytes)
}

pub fn hex_to_base64(hex_string: &str) -> String {
    let bytes = hex_string_to_bytes(hex_string);
    bytes_to_base64(&bytes)
}

#[rustfmt::skip]
pub fn bytes_to_base64(bytes: &[u8]) -> String {
     let mut padding = Vec::new();

    match bytes.len() % 3 {
        1 => {
            padding.push(0);
            padding.push(0);
        }
        2 => padding.push(0),
        _ => (),
    }

    let padded_bytes = [bytes, padding.as_slice()].concat();

    let mut reader = BufReader::new(padded_bytes.as_slice());

    let mut working_buffer: [u8; 3] = [0, 0, 0];
    let mut result: Vec<u8> = Vec::new();

    while let Ok(()) = reader.read_exact(&mut working_buffer) {
        result.push((working_buffer[0] & 0b1111_1100) >> 2);
        result.push((working_buffer[0] & 0b0000_0011) << 4 | (working_buffer[1] & 0b1111_0000) >> 4);
        result.push((working_buffer[1] & 0b0000_1111) << 2 | (working_buffer[2] & 0b1100_0000) >> 6);
        result.push( working_buffer[2] & 0b0011_1111);
    }

    result
        .into_iter()
        .map(|c| BASE_64_TABLE[c as usize])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_base64() {
        assert_eq!(string_to_base64("Ow!"), "T3ch")
    }

    #[test]
    fn test_convert_base64_complex() {
        let test_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex_to_base64(test_string), expected_output);
    }
}
