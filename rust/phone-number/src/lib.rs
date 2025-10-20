pub fn number(user_number: &str) -> Option<String> {
    let mut number: String = user_number.chars().filter(char::is_ascii_digit).collect();
    if number.len() == 11 && &number[0..1] == "1" {
        number.remove(0);
    }
    if number.len() != 10
        || ["0", "1"].contains(&&number[0..1])
        || ["0", "1"].contains(&&number[3..4])
    {
        None
    } else {
        Some(number)
    }
}
