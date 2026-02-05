use std::collections::HashMap;

const DISCOUNT: [u32; 5] = [0, 10, 30, 80, 125];

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts = HashMap::new();
    for book in books {
        *counts.entry(book).or_insert(0) += 1;
    }
    let mut counts: Vec<_> = counts.values().copied().collect();
    counts.sort_unstable_by(|a, b| b.cmp(a));

    for i in 0..counts.len().saturating_sub(1) {
        counts[i] -= counts[i + 1];
    }

    if counts.len() >= 5 {
        let m = counts[2].min(counts[4]);
        counts[2] -= m;
        counts[4] -= m;
        counts[3] += 2 * m;
    }

    let deduction: u32 = counts.iter().zip(DISCOUNT).map(|(x, y)| x * y).sum();
    (books.len() as u32 * 100 - deduction) * 8
}
