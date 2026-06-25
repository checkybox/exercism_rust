pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];
    let mut candidate: u32 = 3;

    while primes.len() <= n as usize {
        let mut is_prime = true;

        for &prime in &primes {
            if prime * prime > candidate {
                break;
            }

            if candidate % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(candidate);
        }

        candidate += 2;
    }

    primes[n as usize]
}
