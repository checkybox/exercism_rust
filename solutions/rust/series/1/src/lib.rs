pub fn series(digits: &str, len: usize) -> Vec<String> {
    let chars = digits.chars().collect::<Vec<_>>();
    chars
        .windows(len)
        .map(|window| window.iter().collect::<String>())
        .collect()
}
