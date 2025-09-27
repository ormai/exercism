const POWER_OF_TENS: [(u64, &str); 7] = [
    (10u64.pow(18), "quintillion"),
    (10u64.pow(15), "quadrillion"),
    (10u64.pow(12), "trillion"),
    (10u64.pow(9), "billion"),
    (10u64.pow(6), "million"),
    (10u64.pow(3), "thousand"),
    (10u64.pow(2), "hundred"),
];

#[rustfmt::skip]
const TENS: [&str; 18] = [
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
    "sixteen", "seventeen", "eighteen", "nineteen", "twenty", "thirty",
    "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const UNITS: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn encode(n: u64) -> String {
    if n == 0 {
        "zero".to_string()
    } else {
        power_of_ten(n).trim_end_matches(['-', ' ']).to_string()
    }
}

fn power_of_ten(n: u64) -> String {
    if let Some(&(pow, name)) = POWER_OF_TENS.iter().find(|(pow, _)| n / pow != 0) {
        format!("{} {name} {}", power_of_ten(n / pow), power_of_ten(n % pow))
    } else {
        match n {
            1..10 => UNITS[n as usize].to_string(),
            10..20 => TENS[(n % 10) as usize].to_string(),
            _ => format!(
                "{}-{}",
                if n / 10 > 0 {
                    TENS[8 + (n / 10) as usize]
                } else {
                    ""
                },
                UNITS[(n % 10) as usize],
            ),
        }
    }
}
