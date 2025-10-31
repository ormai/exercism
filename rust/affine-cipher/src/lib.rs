#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Size of the Latin alphabet
const M: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`).
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if mmi(a, M).is_some() {
        let encoded = transform(&plaintext.to_lowercase(), |c| {
            (a * (c - b'a') as i32 + b) % M
        });
        Ok(chunk_by(&encoded, 5))
    } else {
        Err(AffineCipherError::NotCoprime(a))
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`).
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if let Some(mmi_a) = mmi(a, M) {
        Ok(transform(ciphertext, |c| {
            euclidean_mod(mmi_a * ((c - b'a') as i32 - b), M)
        }))
    } else {
        Err(AffineCipherError::NotCoprime(a))
    }
}

fn transform(s: &str, f: impl Fn(u8) -> i32) -> String {
    s.bytes()
        .filter_map(|c| match c {
            b'a'..=b'z' => Some((f(c) as u8 + b'a') as char),
            b'0'..=b'9' => Some(c as char),
            _ => None,
        })
        .collect()
}

fn chunk_by(s: &str, n: usize) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i < s.len() - 1 && (i + 1) % n == 0 {
                vec![c, ' ']
            } else {
                vec![c]
            }
        })
        .collect()
}

fn mmi(a: i32, m: i32) -> Option<i32> {
    (1..m).find(|&x| a * x % m == 1)
}

fn euclidean_mod(a: i32, b: i32) -> i32 {
    (a % b + b) % b
}
