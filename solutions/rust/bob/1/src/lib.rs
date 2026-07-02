pub fn reply(message: &str) -> &str {
    if is_empty(message) {
        "Fine. Be that way!"
    } else if is_question(message) && is_shouting(message) {
        "Calm down, I know what I'm doing!"
    } else if is_question(message) {
        "Sure."
    } else if is_shouting(message) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

fn is_empty(message: &str) -> bool {
    message.trim().is_empty()
}

fn is_question(message: &str) -> bool {
    message.trim().chars().last() == Option::from('?')
}

fn is_shouting(message: &str) -> bool {
    let has_letters = message.chars().any(|c| c.is_alphabetic());
    let all_uppercase = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase());

    has_letters && all_uppercase
}
