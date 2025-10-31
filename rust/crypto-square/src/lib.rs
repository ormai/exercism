pub fn encrypt(input: &str) -> String {
    let norm: Vec<_> = input
        .to_lowercase()
        .bytes()
        .filter(u8::is_ascii_alphanumeric)
        .collect();

    let c = (norm.len() as f64).sqrt().ceil() as usize;
    let r = (0..=c).find(|r| c * r >= norm.len()).unwrap();

    (0..c)
        .map(|j| {
            (0..r)
                .map(|i| *norm.get(i * c + j).unwrap_or(&b' ') as char)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
        .to_string()
}
