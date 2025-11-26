use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (3..=(sum - 3) / 2)
        .filter_map(|a| {
            let a_sq = a * a;
            (a + 1..=(sum - 1 - a) / 2).find_map(|b| {
                let c = sum - a - b;
                if a_sq + b * b == c * c {
                    Some([a, b, c])
                } else {
                    None
                }
            })
        })
        .collect()
}
