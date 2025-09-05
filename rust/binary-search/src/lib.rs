use std::cmp::Ordering::{Equal, Greater, Less};

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let (mut l, mut r) = (0, array.len());
    while l < r {
        let m = (l + r) / 2;
        match array[m].cmp(&key) {
            Less => l = m + 1,
            Greater => r = m,
            Equal => return Some(m),
        }
    }
    None
}
