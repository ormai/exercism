#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|&v| vlq(v)).collect()
}

fn vlq(mut n: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    if n == 0 {
        bytes.push(0);
    }
    while n > 0 {
        bytes.push((n & 0x7F) as u8);
        n >>= 7;
    }
    bytes.reverse();
    let n = bytes.len();
    for byte in &mut bytes[..n - 1] {
        *byte |= 0x80;
    }
    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut values = vec![];
    for sequence in bytes.split_inclusive(|byte| byte & 0x80 == 0) {
        if sequence.last().map_or(true, |byte| byte & 0x80 != 0) {
            return Err(Error::IncompleteNumber);
        }
        values.push(
            sequence
                .iter()
                .fold(0, |acc, byte| (acc << 7) + (byte & 0x7F) as u32),
        );
    }
    Ok(values)
}
