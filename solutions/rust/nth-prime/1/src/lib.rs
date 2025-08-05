pub fn nth(n: u32) -> u32 {
    let limit = if n < 6 {
        15
    } else {
        let n_f64 = n as f64;
        let limit_f64 = n_f64 * (n_f64.ln() + n_f64.ln().ln());
        (limit_f64 * 1.2) as u32
    };

    let primes = sieve_of_eratosthenes(limit);
    
    primes[n as usize]
}

fn sieve_of_eratosthenes(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();

    let limit_usize = limit as usize;

    let mut is_prime = vec![true; limit_usize + 1];

    if limit_usize >= 0 { is_prime[0] = false; }
    if limit_usize >= 1 { is_prime[1] = false; }


    for i in 2..=limit.isqrt() {
        if is_prime[i as usize] {
            for j in (i * i..=limit).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }

    for p in 2..=limit {
        if is_prime[p as usize] {
            primes.push(p);
        }
    }

    primes
}
