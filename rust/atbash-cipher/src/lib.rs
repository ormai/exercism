pub fn encode(plain: &str) -> String {
    let encoded = atbash(plain);
    encoded
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i < encoded.len() - 1 && (i + 1) % 5 == 0 {
                vec![c, ' ']
            } else {
                vec![c]
            }
        })
        .collect()
}

pub fn decode(cipher: &str) -> String {
    atbash(cipher)
}

fn atbash(s: &str) -> String {
    s.bytes()
        .filter_map(|c| match c {
            b'a'..=b'z' | b'A'..=b'Z' => Some((b'z' - c.to_ascii_lowercase() + b'a') as char),
            b'0'..=b'9' => Some(c as char),
            _ => None,
        })
        .collect()
}
