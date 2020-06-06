pub fn hamming_distance<B: AsRef<[u8]>>(bytes_1: B, bytes_2: B) -> usize {
    let bytes_1 = bytes_1.as_ref();
    let bytes_2 = bytes_2.as_ref();

    let length = bytes_1.len();

    if bytes_2.len() != length {
        panic!("Strings must be the same length");
    }

    let distance = bytes_1.iter()
        .zip(bytes_2.iter())
        .map(|(b1, b2)| byte_distance(*b1, *b2))
        .sum();

    distance
}

fn byte_distance(b1: u8, b2: u8) -> usize {
    let mut distance = 0;

    for shift in 0..8 {
        let mask = 0b0000_0001 << shift;
        if (b1 & mask) != (b2 & mask) {
            distance += 1;
        }
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";

        let distance = hamming_distance(s1, s2);
        assert_eq!(distance, 37);
    }
}
