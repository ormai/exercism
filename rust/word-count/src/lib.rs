use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| c.is_ascii_whitespace() || c != '\'' && c.is_ascii_punctuation())
        .map(|word| word.trim_matches('\''))
        .filter(|&word| !word.is_empty())
        .fold(HashMap::new(), |mut count, word| {
            *count.entry(word.to_lowercase()).or_default() += 1;
            count
        })
}
