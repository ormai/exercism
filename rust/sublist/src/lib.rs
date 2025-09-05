#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    let a = Vec::from_iter(first_list.iter());
    let b = Vec::from_iter(second_list.iter());
    if a.is_empty() || b.windows(a.len()).any(|win| win == a) {
        Comparison::Sublist
    } else if b.is_empty() || a.windows(b.len()).any(|win| win == b) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
