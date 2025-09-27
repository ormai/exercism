use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    fn new(value: u64, factors: (u64, u64)) -> Self {
        Self {
            value,
            factors: HashSet::from([factors]),
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_palindrome: Option<Palindrome> = None;
    let mut max_palindrome: Option<Palindrome> = None;

    for a in min..=max {
        for b in a..=max {
            let product = a * b;
            if let Some(min) = &mut min_palindrome
                && min.value == product
            {
                min.factors.insert((a, b));
            } else if let Some(max) = &mut max_palindrome
                && max.value == product
            {
                max.factors.insert((a, b));
            } else {
                let num = product.to_string();
                if num[..num.len() / 2]
                    .chars()
                    .zip(num[num.len() / 2..].chars().rev())
                    .all(|(c1, c2)| c1 == c2)
                {
                    if min_palindrome
                        .as_ref()
                        .is_none_or(|old_best| old_best.value > product)
                    {
                        min_palindrome = Some(Palindrome::new(product, (a, b)));
                    }
                    if max_palindrome
                        .as_ref()
                        .is_none_or(|old_best| old_best.value < product)
                    {
                        max_palindrome = Some(Palindrome::new(product, (a, b)));
                    }
                }
            }
        }
    }

    Some((min_palindrome?, max_palindrome?))
}
