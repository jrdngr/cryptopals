pub fn from_hex(s: &str) -> Result<Vec<u8>, ()> {
    let bytes = s.as_bytes();
    Ok(Vec::from(bytes))
}

pub fn from_base64(s: &str) -> Result<Vec<u8>, ()> {
    let bytes = s.as_bytes();
    Ok(Vec::from(bytes))
}

pub fn print_hex(value: &[u8]) {
    dbg!(value);
}

pub fn print_base64(value: &[u8]) {
    dbg!(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        let result = from_hex("0123ABC").unwrap();
        dbg!(result);
    }
}
