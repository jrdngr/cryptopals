use std::io::{Read, BufReader};
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref BASE_64_TABLE: Vec<char> = {
        vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
            'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
            'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
            'w', 'x', 'y', 'z', '0', '1', '2', '3',
            '4', '5', '6', '7', '8', '9', '+', '/',
        ] 
    };

    static ref ASCII_HEX_TABLE: HashMap<u8, u8> = {
        let mut table = HashMap::new();

        table.insert(48, 0);
        table.insert(49, 1);
        table.insert(50, 2);
        table.insert(51, 3);
        table.insert(52, 4);
        table.insert(53, 5);
        table.insert(54, 6);
        table.insert(55, 7);
        table.insert(56, 8);
        table.insert(57, 9);

        table.insert(65, 10);
        table.insert(66, 11);
        table.insert(67, 12);
        table.insert(68, 13);
        table.insert(69, 14);
        table.insert(70, 15);

        table.insert(97, 10);
        table.insert(98, 11);
        table.insert(99, 12);
        table.insert(100, 13);
        table.insert(101, 14);
        table.insert(102, 15);

        table
    };
}

const MASK_1: u8 = 0b1111_1100;
const MASK_2: u8 = 0b1111_0000;
const MASK_3: u8 = 0b0011_1111;

pub fn string_to_base64(string: &str) -> String {
    let bytes = string.bytes().collect::<Vec<u8>>();
    bytes_to_base64(&bytes)
}

pub fn hex_to_base64(hex_string: &str) -> String {
    let hex_bytes: Vec<u8> = hex_string
        .bytes()
        .map(|b| ASCII_HEX_TABLE[&b])
        .collect();
    
    let bytes: Vec<u8> = hex_bytes
        .as_slice()
        .chunks(2)
        .map(|c| (c[0] << 4) | (c[1]))
        .collect();

    bytes_to_base64(&bytes)
}

pub fn bytes_to_base64(bytes: &[u8]) -> String {
    let length = bytes.len();
    let mut padding = Vec::new();

    match length {
        1 => {
            padding.push(0);
            padding.push(0);
        },
        2 => padding.push(0),
        _ => (),
    }

    let padded_bytes = [bytes, padding.as_slice()].concat();

    let mut reader = BufReader::new(padded_bytes.as_slice());

    let mut working_buffer: [u8; 3] = [0, 0, 0];
    let mut result: Vec<u8> = Vec::new();

    while let Ok(()) = reader.read_exact(&mut working_buffer) {
        result.push((working_buffer[0] & MASK_1) >> 2);
        result.push((working_buffer[0] & !MASK_1) << 4 | (working_buffer[1] & MASK_2) >> 4);
        result.push((working_buffer[1] & !MASK_2) << 2 | (working_buffer[2] & !MASK_3 >> 4));
        result.push(working_buffer[2] & MASK_3);
    }

    result.into_iter().map(|c| BASE_64_TABLE[c as usize]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!("T3ch", string_to_base64("Ow!"))
    }
}
