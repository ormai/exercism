pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        None
    } else if n == 1 {
        Some(0)
    } else {
        Some(1 + collatz(if n % 2 == 0 { n / 2 } else { 3 * n + 1 })?)
    }
}
