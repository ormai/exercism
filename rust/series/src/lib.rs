pub fn series(digits: &str, len: usize) -> Vec<String> {
    Vec::from_iter(digits.chars())
        .windows(len)
        .map(|w| w.iter().collect())
        .collect()
}
