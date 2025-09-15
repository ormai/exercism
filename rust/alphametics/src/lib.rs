use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// https://exercism.org/tracks/rust/exercises/alphametics/solutions/iamhere2

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let firsts: HashSet<_> = input
        .split(&['+', '='])
        .filter_map(|s| s.trim().chars().next())
        .collect();

    let mut factors = HashMap::new();
    let mut sign = -1;
    let mut pos = 0;
    for c in input.chars().filter(|c| !c.is_whitespace()).rev() {
        match c {
            '=' => {
                sign = 1;
                pos = 0
            }
            '+' => pos = 0,
            _ => {
                *factors.entry(c).or_insert(0) += sign * 10_i64.pow(pos);
                pos += 1;
            }
        }
    }
    let (letters, factors): (Vec<_>, Vec<_>) =
        factors.into_iter().sorted_by_key(|(_, v)| -v.abs()).unzip();

    (0..10).permutations(letters.len()).find_map(|p| {
        if p.iter()
            .enumerate()
            .map(|(i, v)| v * factors[i])
            .sum::<i64>()
            == 0
            && !p
                .iter()
                .enumerate()
                .any(|(i, &v)| v == 0 && firsts.contains(&letters[i]))
        {
            Some(
                p.iter()
                    .enumerate()
                    .map(|(i, &v)| (letters[i], v as u8))
                    .collect(),
            )
        } else {
            None
        }
    })
}
