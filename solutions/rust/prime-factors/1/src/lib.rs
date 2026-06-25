pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut number: u64 = n;
    let mut divisor: u64 = 2;

    while number > 1 {
        if number.is_multiple_of(divisor) {
            number /= divisor;
            factors.push(divisor);
        } else {
            divisor += 1;
        }
    }

    factors
}
