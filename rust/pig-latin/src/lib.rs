const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn translate(input: &str) -> String {
    input
        .split(" ")
        .map(|word| {
            if word.starts_with(VOWELS) || word.starts_with("xr") || word.starts_with("yt") {
                format!("{word}ay")
            } else if let Some(qu) = word.find("qu")
                && !word[0..qu].chars().any(|c| VOWELS.contains(&c))
            {
                format!("{}{}ay", &word[qu + 2..], &word[..qu + 2])
            } else if let Some(y) = word.get(1..).and_then(|word| word.find('y'))
                && !word[..y + 1].chars().any(|c| VOWELS.contains(&c))
            {
                format!("{}{}ay", &word[y + 1..], &word[..y + 1])
            } else if let Some(vowel) = word.find(|c| VOWELS.contains(&c)) {
                format!("{}{}ay", &word[vowel..], &word[..vowel])
            } else {
                unreachable!()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
