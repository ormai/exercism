use num::{BigInt, Num};
use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, PartialEq)]
pub struct Decimal {
    mantissa: BigInt,
    exponent: usize,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (int, frac) = input
            .split_once('.')
            .map(|(i, f)| (i, f.trim_end_matches('0')))
            .unwrap_or((input, ""));
        Some(Self {
            mantissa: BigInt::from_str_radix((int.to_string() + frac).as_str(), 10).ok()?,
            exponent: frac.len(),
        })
    }

    fn normalize(self) -> Self {
        let repr = self.mantissa.to_string();
        let zeroes = repr.len() - repr.trim_end_matches('0').len();
        Self {
            exponent: self.exponent - zeroes,
            mantissa: self.mantissa / BigInt::from(10).pow(zeroes.try_into().unwrap()),
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.mantissa.sign() != other.mantissa.sign() {
            self.mantissa.sign().partial_cmp(&other.mantissa.sign())
        } else if self.exponent == other.exponent {
            self.mantissa.partial_cmp(&other.mantissa)
        } else {
            let a = &self.mantissa * BigInt::from(10).pow(other.exponent.try_into().ok()?);
            let b = &other.mantissa * BigInt::from(10).pow(self.exponent.try_into().ok()?);
            a.partial_cmp(&b)
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (min, max) = if self.exponent >= rhs.exponent {
            (self, rhs)
        } else {
            (rhs, self)
        };
        Self {
            exponent: min.exponent,
            mantissa: min.mantissa
                + max.mantissa * BigInt::from(10).pow((min.exponent - max.exponent) as u32),
        }
        .normalize()
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + Self {
            mantissa: -rhs.mantissa,
            exponent: rhs.exponent,
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            exponent: self.exponent + rhs.exponent,
            mantissa: self.mantissa * rhs.mantissa,
        }
        .normalize()
    }
}
