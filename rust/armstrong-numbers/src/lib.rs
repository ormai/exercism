pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    num == digits
        .iter()
        .map(|d| d.pow(digits.len() as u32))
        .sum::<u32>()
}
