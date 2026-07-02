pub fn raindrops(n: u32) -> String {
    let result: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .filter_map(|(divisor, sound)| {
            n.is_multiple_of(divisor).then_some(sound)
        })
        .collect::<String>();

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
