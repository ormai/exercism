pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut primes: Vec<u64> = (2..=upper_bound).rev().collect();
    while let Some(prime) = primes.pop() {
        res.push(prime);
        primes.retain(|n| n % prime != 0);
    }
    res
}
