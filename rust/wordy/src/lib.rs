type Operation = fn(i32, i32) -> Option<i32>;

pub fn answer(command: &str) -> Option<i32> {
    let expr = command.trim_start_matches("What is ").trim_end_matches('?');

    if let Ok(num) = expr.parse::<i32>() {
        return Some(num);
    }

    let mut pairs: [(&str, Operation); 5] = [
        (" plus ", i32::checked_add),
        (" minus ", i32::checked_sub),
        (" multiplied by ", i32::checked_mul),
        (" divided by ", i32::checked_div),
        (" raised to the ", pow),
    ];

    pairs.sort_by(|(a, _), (b, _)| expr.find(b).cmp(&expr.find(a)));
    pairs.iter().fold(None, |output, (s, op)| {
        output.or_else(|| {
            expr.rsplit_once(s).and_then(|(lhs, rhs)| {
                let rhs = if s.starts_with(" raised") {
                    &rhs[..rhs.find(|c: char| !c.is_ascii_digit())?]
                } else {
                    rhs
                };
                answer(lhs).zip(answer(rhs)).and_then(|(a, b)| op(a, b))
            })
        })
    })
}

fn pow(base: i32, exponent: i32) -> Option<i32> {
    Some(u32::pow(base.try_into().ok()?, exponent.try_into().ok()?) as i32)
}
