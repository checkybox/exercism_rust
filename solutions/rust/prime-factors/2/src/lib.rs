pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut number = n;

    while number.is_multiple_of(2) {
        factors.push(2);
        number /= 2;
    }

    let mut divisor = 3;
    while divisor * divisor <= number {
        while number.is_multiple_of(divisor) {
            factors.push(divisor);
            number /= divisor;
        }

        divisor += 2;
    }

    if number > 1 {
        factors.push(number);
    }

    factors
}
