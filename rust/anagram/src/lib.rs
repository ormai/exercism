use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut word = Vec::from_iter(word.to_lowercase().chars());
    possible_anagrams
        .iter()
        .filter(|a| {
            let mut a = Vec::from_iter(a.to_lowercase().chars());
            if a == word {
                return false;
            }
            a.sort();
            word.sort();
            a == word
        })
        .cloned()
        .collect()
}
