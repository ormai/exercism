pub fn rotate(input: &str, key: u8) -> String {
    input
        .bytes()
        .map(|c| match c {
            b'a'..=b'z' => (c - b'a' + key) % 26 + b'a',
            b'A'..=b'Z' => (c - b'A' + key) % 26 + b'A',
            _ => c,
        } as char)
        .collect()
}
