pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None
    }

    let mut count = 0;
    let mut temp = n;
    while temp != 1 {
        temp = if temp.is_multiple_of(2) {
            temp / 2
        } else {
            temp.checked_mul(3)?.checked_add(1)?
        };
        count += 1;
    }

    Some(count)
}
