const MOD: u64 = 1_000_000_007;

fn mod_pow(base: u64, exp: u64) -> u64 {
    let mut result = 1;
    let mut base = base % MOD;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 != 0 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp /= 2;
    }
    result
}

// https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
fn mod_inv(a: u64) -> u64 {
    mod_pow(a, MOD - 2)
}
