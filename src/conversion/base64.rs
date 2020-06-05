use std::io::{BufReader, Read};

use crate::conversion::hex::{hex_string_to_bytes, bytes_to_hex_string};

#[rustfmt::skip]
const BASE_64_TABLE: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 
    'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', 
    '8', '9', '+', '/', '='
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
    let padding_count = match bytes.len() % 3 {
        0 => 0,
        1 => 2,
        2 => 1,
        _ => unreachable!(),
    };

    let padding = match padding_count {
        1 => vec![0],
        2 => vec![0, 0],
        _ => Vec::new(),
    };
    
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

    let end = result.len() - 1;

    for i in 0..padding_count {
        result[end - i] = 64;
    }

    result
        .into_iter()
        .map(|c| BASE_64_TABLE[c as usize])
        .collect()
}

pub fn base_64_character_to_byte(c: u8) -> u8 {
    if c >= 65 && c <= 90 {
        c - 65
    } else if c >= 97 && c <= 122 {
        c - 71
    } else if c >= 48 && c <= 57 {
        c + 4
    } else if c == 43 {
        62
    } else if c == 47 {
        63
    } else if c == 61 {
        0
    } else {
        unreachable!()
    }
}

pub fn base64_to_hex_string<B: AsRef<[u8]>>(base64: B) -> String {
    bytes_to_hex_string(&base64_to_bytes(base64))
}

#[rustfmt::skip]
pub fn base64_to_bytes<B: AsRef<[u8]>>(base64: B) -> Vec<u8> {
    let bytes: Vec<u8> = base64.as_ref()
        .iter()
        .cloned()
        .filter(|c| *c != 10 && *c != 61) // filter out line feed and =
        .map(base_64_character_to_byte)
        .collect();

    let padding_count = match bytes.len() % 4 {
        0 => 0,
        2 => 2, 
        3 => 1,
        1 => panic!("Invalid base64"),
        _ => unreachable!(),
    };

    let padding = match padding_count {
        1 => vec![0],
        2 => vec![0, 0],
        _ => Vec::new(),
    };
    
    let padded_bytes = [bytes, padding].concat();

    let mut reader = BufReader::new(padded_bytes.as_slice());

    let mut working_buffer: [u8; 4] = [0, 0, 0, 0];
    let mut result = Vec::new();
    
    while let Ok(()) = reader.read_exact(&mut working_buffer) {
        result.push(working_buffer[0] << 2 | (working_buffer[1] & 0b0011_0000) >> 4);
        result.push((working_buffer[1] & 0b0000_1111) << 4 | (working_buffer[2] & 0b0011_1100) >> 2);
        result.push((working_buffer[2] & 0b0000_0011) << 6 | working_buffer[3]);
    }

    let result_length = result.len() - padding_count;

    result.into_iter().take(result_length).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_string_to_base64_padding() {
        assert_eq!(string_to_base64("Ow!"), "T3ch")
    }

    #[test]
    fn test_convert_string_to_base64_padding_0() {
        assert_eq!(string_to_base64("Man"), "TWFu")
    }

    #[test]
    fn test_convert_string_to_base64_padding_1() {
        assert_eq!(string_to_base64("Ma"), "TWE=");
    }

    #[test]
    fn test_convert_string_to_base64_padding_2() {
        assert_eq!(string_to_base64("M"), "TQ==");
    }

    #[test]
    fn test_convert_hex_to_base64_complex() {
        let test_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(hex_to_base64(test_string), expected_output);
    }

    #[test]
    fn test_base64_to_bytes_same_input_padding_0() {
        let original = "Man";

        let base64 = string_to_base64(original);
        let bytes = base64_to_bytes(base64);

        assert_eq!(String::from_utf8_lossy(&bytes), original);
    }

    #[test]
    fn test_base64_to_bytes_same_input_padding_1() {
        let original = "Ma";

        let base64 = string_to_base64(original);
        let bytes = base64_to_bytes(base64);

        assert_eq!(String::from_utf8_lossy(&bytes), original);
    }

    #[test]
    fn test_base64_to_bytes_same_input_padding_2() {
        let original = "M";

        let base64 = string_to_base64(original);
        let bytes = base64_to_bytes(base64);

        assert_eq!(String::from_utf8_lossy(&bytes), original);
    }
}

