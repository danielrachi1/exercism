pub fn factors(n: u64) -> Vec<u64> {
    let mut n_mut = n;
    let mut factors = Vec::new();
    let mut divisor = 2;

    while n_mut != 1 {
        if n_mut % divisor == 0 {
            factors.push(divisor);
            n_mut /= divisor;
        } else {
            divisor += 1;
        }
    }

    factors
}
