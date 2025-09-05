pub fn reply(message: &str) -> &str {
    let uppercase_letters = || {
        message.chars().any(|c| c.is_alphabetic())
            && !message
                .chars()
                .any(|c| c.is_alphabetic() && c.is_lowercase())
    };

    if message.chars().all(|c| c.is_whitespace()) {
        "Fine. Be that way!"
    } else if message.trim().ends_with("?") {
        if uppercase_letters() {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if uppercase_letters() {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
