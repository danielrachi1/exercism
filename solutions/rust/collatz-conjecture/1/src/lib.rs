pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = 0;
    let mut n = n;

    if (n == 0) {return None}
    
    while n != 1 {
        match n % 2 {
            0 => n = n/2,
            1 => n = n*3 + 1,
            _ => continue,
        }
        steps += 1;
    }

    Some(steps)
}
