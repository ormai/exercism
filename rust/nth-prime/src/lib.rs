pub fn nth(n: u32) -> u32 {
    let n = n as usize;
    let mut num = 1;
    let mut count = 0;
    while count <= n {
        num += 1;
        if is_prime(num) {
            count += 1;
        }
    }
    num
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let upper_bound = (n as f64).sqrt() as u32;
    for divisor in (5..=upper_bound).step_by(6) {
        if n % divisor == 0 || n % (divisor + 2) == 0 {
            return false;
        }
    }

    true
}
