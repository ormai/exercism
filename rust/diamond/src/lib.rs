pub fn get_diamond(c: char) -> Vec<String> {
    ('A'..=c)
        .chain(('A'..c).rev())
        .map(|r| {
            ('A'..=c)
                .rev()
                .chain('B'..=c)
                .map(|c| if r == c { r } else { ' ' })
                .collect()
        })
        .collect()
}
