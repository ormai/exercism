use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        fn arab_to_roman(digit: usize, one: &str, five: &str, ten: &str) -> String {
            match digit {
                0 => "".to_owned(),
                1..=3 => one.repeat(digit),
                4 => format!("{one}{five}"),
                5 => five.to_owned(),
                6..=8 => format!("{five}{}", one.repeat(digit - 5)),
                9 => format!("{one}{ten}"),
                other => unreachable!("Only 1-9 digits are expected, got {other}"),
            }
        }

        write!(
            f,
            "{}{}{}{}",
            "M".repeat((self.0 % 10000 / 1000) as usize),
            arab_to_roman((self.0 % 1000 / 100) as usize, "C", "D", "M"),
            arab_to_roman((self.0 % 100 / 10) as usize, "X", "L", "C"),
            arab_to_roman((self.0 % 10) as usize, "I", "V", "X")
        )
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self(num)
    }
}
