pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|ch: char| !ch.is_alphabetic() && ch != '\'')
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            let mut chars = word.chars();
            let first = chars.next().unwrap().to_ascii_uppercase();

            if chars.all(|ch| ch.is_ascii_uppercase()) {
                return std::iter::once(first).chain(vec![])
            }

            let rest = chars
                .filter(|ch| ch.is_ascii_uppercase())
                .collect::<Vec<char>>();
            return std::iter::once(first).chain(rest)
        })
        .collect::<String>()
}
