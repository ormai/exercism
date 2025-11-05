#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        Ok(1)
    } else {
        string_digits
            .chars()
            .map(|d| d.to_digit(10).map(u64::from).ok_or(Error::InvalidDigit(d)))
            .collect::<Result<Vec<_>, _>>()?
            .windows(span)
            .map(|win| win.iter().product())
            .max()
            .ok_or(Error::SpanTooLong)
    }
}
