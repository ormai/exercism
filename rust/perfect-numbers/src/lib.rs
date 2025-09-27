use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num > 0 {
        Some(
            match num.cmp(&(1..=num / 2).filter(|&div| num.is_multiple_of(div)).sum()) {
                Ordering::Equal => Classification::Perfect,
                Ordering::Less => Classification::Abundant,
                Ordering::Greater => Classification::Deficient,
            },
        )
    } else {
        None
    }
}
