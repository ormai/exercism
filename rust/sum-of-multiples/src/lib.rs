use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();

    for factor in factors.iter().filter(|x| **x != 0) {
        for multiple in 1..limit {
            if multiple % factor == 0 {
                multiples.insert(multiple);
            }
        }
    }

    multiples.iter().sum()
}
