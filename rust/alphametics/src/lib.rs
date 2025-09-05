use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (a, raw) = input.split_once(" + ")?;
    let (b, c) = raw.split_once(" == ")?;

    let mut assignment = HashMap::new();

    // If the result is longer than the two addenda, then there is a carry and the leftmost digit
    // of the result must be 1.
    if c.len() > (a.len()).max(b.len()) {
        assignment.insert(c.chars().next().unwrap(), 1);
    }

    println!("{a} {b} {c}",);

    Some(assignment)
}
