pub fn hex_string_to_bytes(hex_string: &str) -> Vec<u8> {
    let mut hex_bytes: Vec<u8> = hex_string
        .bytes()
        .map(ascii_hex_to_byte)
        .collect();

    if hex_bytes.len() % 2 == 1 {
        hex_bytes.insert(0, 0);
    }

    hex_bytes
        .as_slice()
        .chunks(2)
        .map(|c| (c[0] << 4) | (c[1]))
        .collect()
}

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let result: Vec<[char; 2]> = bytes
        .iter()
        .map(|b| byte_to_ascii_hex_pair(*b))
        .collect();

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

pub fn byte_to_ascii_hex_pair(byte: u8) -> [char; 2] {
    let c1 = (byte & 0b1111_0000) >> 4;
    let c2 = byte & 0b0000_1111;
    [byte_to_ascii_hex(c1), byte_to_ascii_hex(c2)]
}

pub fn byte_to_ascii_hex(byte: u8) -> char {
    if byte <= 9 {
        (byte + 48) as char
    } else if byte >=10 && byte <= 15 {
        (byte + 87) as char
    } else {
        panic!("Invalid hex value: {}", byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes_even() {
        let test_string = "1c0111001f010100061a024b53535009181c";
        let expected_output = vec![28, 1, 17, 0, 31, 1, 1, 0, 6, 26, 2, 75, 83, 83, 80, 9, 24, 28];
        assert_eq!(hex_string_to_bytes(test_string), expected_output);
    }

    #[test]
    fn test_hex_to_bytes_odd() {
        let test_string = "1c0111001f010100061a024b53535009181c1";
        let expected_output = vec![1, 192, 17, 16, 1, 240, 16, 16, 0, 97, 160, 36, 181, 53, 53, 0, 145, 129, 193];
        assert_eq!(hex_string_to_bytes(test_string), expected_output);
    }

    #[test]
    fn test_bytes_to_hex_even() {
        let test_bytes = vec![28, 1, 17, 0, 31, 1, 1, 0, 6, 26, 2, 75, 83, 83, 80, 9, 24, 28];
        let expected_output = "1c0111001f010100061a024b53535009181c";
            assert_eq!(bytes_to_hex_string(&test_bytes), expected_output);
    }

    #[test]
    fn test_bytes_to_hex_odd() {
        let test_bytes = vec![1, 192, 17, 16, 1, 240, 16, 16, 0, 97, 160, 36, 181, 53, 53, 0, 145, 129, 193];
        let expected_output = "1c0111001f010100061a024b53535009181c1";
            assert_eq!(bytes_to_hex_string(&test_bytes), expected_output);
    }
}
