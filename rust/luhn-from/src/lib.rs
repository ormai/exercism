pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut n = 0;
        let mut acc = 0;

        for c in self.0.chars().rev() {
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

impl<T: ToString> From<T> for Luhn {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
