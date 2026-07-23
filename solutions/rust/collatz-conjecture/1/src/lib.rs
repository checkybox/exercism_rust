pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None
    }

    let mut count = 0;
    let mut temp = n;
    while temp != 1 {
        if temp.is_multiple_of(2) {
            temp /= 2;
        } else {
            temp = temp * 3 + 1;
        }
        count += 1;
    }

    Some(count)
}
