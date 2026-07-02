pub fn raindrops(n: u32) -> String {
    let result: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|(divisor, _)| n.is_multiple_of(*divisor))
        .map(|&(_, string)| string)
        .collect::<String>();

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
