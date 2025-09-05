pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut div = 2;
    while n > 1 {
        if n % div == 0 {
            factors.push(div);
            n /= div;
        } else {
            div += 1;
        }
    }
    factors
}
