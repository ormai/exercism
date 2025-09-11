use itertools::Itertools;
use std::cmp;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .chunk_by(|&c| c)
        .into_iter()
        .map(|(c, group)| match group.count() {
            1 => c.to_string(),
            count => format!("{count}{c}"),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold((String::new(), 0), |(acc, n), c| {
            if let Some(d) = c.to_digit(10) {
                (acc, n * 10 + d)
            } else {
                (
                    acc + c.to_string().repeat(cmp::max(1, n) as usize).as_str(),
                    0,
                )
            }
        })
        .0
}
