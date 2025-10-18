pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let mut n = 0;
        let mut acc = 0;

        for c in self.to_string().chars().rev() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                acc += if n & 1 == 1 {
                    let digit = digit + digit;
                    if digit > 9 { digit - 9 } else { digit }
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
}
