/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if !isbn.chars().all(|c| c.is_ascii_digit() || c == '-' || c == 'X') {
        return false;
    }

    let isbn: Vec<char> = isbn
        .chars()
        .filter(|&c| c == 'X' || c.is_ascii_digit())
        .collect();

    if isbn.contains(&'X') && isbn[isbn.len() - 1] != 'X' {
        return false;
    }

    isbn.len() == 10
        && (1..=10).rev().zip(isbn.iter()).fold(0, |acc, (i, &digit)| {
            acc + (if digit == 'X' {
                10
            } else {
                digit.to_digit(10).unwrap()
            } * i as u32)
        }) % 11
            == 0
}
