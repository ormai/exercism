use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let candidate: Vec<char> = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    let letters = candidate.len();
    HashSet::<char>::from_iter(candidate).len() == letters
}
