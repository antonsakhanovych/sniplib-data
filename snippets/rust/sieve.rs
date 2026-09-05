fn sieve(n: usize) -> (Vec<bool>, Vec<usize>) {
    let mut is_prime = vec![true; n + 1];
    let mut spf = vec![0; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n.isqrt() {
        if !is_prime[i] {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            is_prime[j] = false;
            if spf[j] == 0 {
                spf[j] = i;
            }
        }
    }
    for i in 2..=n {
        if is_prime[i] {
            spf[i] = i;
        }
    }
    (is_prime, spf)
}

fn factorize(x: usize, spf: &[usize]) -> Vec<(usize, u32)> {
    let mut result = Vec::new();
    let mut x = x;
    while x > 1 {
        let prime = spf[x];
        let mut exponent = 0;
        while x % prime == 0 {
            x /= prime;
            exponent += 1;
        }
        result.push((prime, exponent));
    }
    result
}
