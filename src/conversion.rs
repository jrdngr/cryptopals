use std::io::{Read, BufReader};

const MASK_1: u8 = 0b11111100;
const MASK_2: u8 = 0b11110000;
const MASK_3: u8 = 0b00111111;

pub fn hex_to_base64(hex_string: &str) -> String {
    let mut bytes = hex_string.bytes().collect::<Vec<u8>>();
    
    let length = bytes.len();
    match length {
        1 => {
            bytes.push(0);
            bytes.push(0);
        },
        2 => bytes.push(0),
        _ => (),
    }

    let mut reader = BufReader::new(bytes.as_slice());

    let mut working_buffer: [u8; 3] = [0, 0, 0];
    let mut result: Vec<u8> = Vec::new();

    while let Ok(()) = reader.read_exact(&mut working_buffer) {
        result.push(working_buffer[0] & MASK_1);
        result.push((working_buffer[0] & !MASK_1) << 6 | (working_buffer[1] & MASK_2) >> 4);
        result.push((working_buffer[1] & !MASK_2) << 4 | (working_buffer[2] & !MASK_3 >> 6));
        result.push(working_buffer[2] & MASK_3);
    }

    String::from_utf8(result).unwrap()
}
