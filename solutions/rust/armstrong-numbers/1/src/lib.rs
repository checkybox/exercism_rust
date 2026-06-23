pub fn is_armstrong_number(num: u32) -> bool {
    let chars = num.to_string().chars().collect::<Vec<_>>();
    let length = chars.len();
    let mut sum = 0;

    for char in chars {
        if char.is_ascii_digit() {
            sum += char.to_digit(10).unwrap().pow(length as u32);
        }
    }
    sum == num
}
