/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 {
        return false
    }

    let code = code.trim();
    let mut sum = 0;
    let mut digit_count = 0;

    // iterate from left to right
    for (i, ch) in code.chars().rev().enumerate() {
        if ch == ' ' {
            continue; // skip spaces
        }

        // convert char to digit (0-9)
        let digit = match ch.to_digit(10) {
            Some(d) => d,
            None => return false,
        };

        digit_count += 1;

        // double every second digit from the right (based on digit position, not char position)
        let digit = if (digit_count - 1) % 2 == 1 {
            let doubled = digit * 2;
            if doubled > 9 { doubled - 9 } else { doubled }
        } else {
            digit
        };

        sum += digit;
    }

    digit_count >= 2 && sum % 10 == 0
}
