pub fn hex_string_to_bytes(hex_string: &str) -> Vec<u8> {
    let hex_bytes: Vec<u8> = hex_string
        .bytes()
        .map(ascii_hex_to_byte)
        .collect();

    hex_bytes
        .as_slice()
        .chunks(2)
        .map(|c| (c[0] << 4) | (c[1]))
        .collect()
}

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let result: Vec<[char; 2]> = bytes
        .iter()
        .map(byte_to_ascii_hex)
        .collect();

    dbg!(&result);

    result.iter().flatten().collect()
}

pub fn ascii_hex_to_byte(hex: u8) -> u8 {
    if hex >= 48 && hex <= 57 {
        hex - 48
    } else if hex >= 65 && hex <= 70 {
        hex - 55
    } else if hex >= 97 && hex <= 102 {
        hex - 87
    } else {
        panic!("Invalid hex character: {}", hex);
    }
}

pub fn byte_to_ascii_hex(byte: &u8) -> [char; 2] {
    let c1 = (byte & 0b1111_0000 >> 4) as char;
    let c2 = (byte & 0b0000_1111) as char;
    [c1, c2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        let test_string = "1c0111001f010100061a024b53535009181c";
        let expected_output = vec![28, 1, 17, 0, 31, 1, 1, 0, 6, 26, 2, 75, 83, 83, 80, 9, 24, 28];
        assert_eq!(hex_string_to_bytes(test_string), expected_output);
    }

    #[test]
    fn test_bytes_to_hex() {
        let test_bytes = vec![28, 1, 17, 0, 31, 1, 1, 0, 6, 26, 2, 75, 83, 83, 80, 9, 24, 28];
        let expected_output = "1c0111001f010100061a024b53535009181c";
            assert_eq!(bytes_to_hex_string(&test_bytes), expected_output);
    }
}
