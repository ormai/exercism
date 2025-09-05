/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut n = 0;
    let mut acc = 0;

    for c in code.chars().rev() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            acc += if n & 1 == 1 {
                let digit = digit + digit;
                if digit > 9 {
                    digit - 9
                } else {
                    digit
                }
            } else {
                digit
            };
            n += 1;
        } else if !c.is_whitespace() {
            return false;
        }
    }

    n > 1 && acc % 10 == 0
}
