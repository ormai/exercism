use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    transform(key, s, |c, k| c + k)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    transform(key, s, |c, k| c + 26 - k)
}

pub fn encode_random(s: &str) -> (String, String) {
    let distr = rand::distr::Uniform::new_inclusive('a', 'z').expect("distribution is valid");
    let key: String = rand::rng().sample_iter(distr).take(128).collect();
    let encoded = encode(&key, s).expect("key is valid");
    (key, encoded)
}

fn transform(key: &str, s: &str, shift: impl Fn(u8, u8) -> u8) -> Option<String> {
    if !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase()) {
        Some(
            s.bytes()
                .zip(key.bytes().cycle())
                .map(|(c, k)| (shift(c - b'a', k - b'a') % 26 + b'a') as char)
                .collect(),
        )
    } else {
        None
    }
}
