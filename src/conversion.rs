use std::io::{Read, BufReader};

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
}

const MASK_1: u8 = 0b1111_1100;
const MASK_2: u8 = 0b1111_0000;
const MASK_3: u8 = 0b0011_1111;

pub fn string_to_base64(string: &str) -> String {
    let bytes = string.bytes().collect::<Vec<u8>>();
    bytes_to_base64(&bytes)
}

pub fn hex_to_base64(hex_string: &str) -> String {
    unimplemented!()
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

    print_bits(&result);

    result.into_iter().map(|c| BASE_64_TABLE[c as usize]).collect()
}

fn print_bits(bytes: &[u8]) {
    for byte in bytes {
        print!("{:08b}, ", byte);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!("T3ch", string_to_base64("Ow!"))
    }
}
