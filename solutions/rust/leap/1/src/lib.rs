pub fn is_leap_year(year: u64) -> bool {
    let divisible_by_4 = is_divisible_by(year, 4);
    let divisible_by_100 = is_divisible_by(year, 100);
    let divisible_by_400 = is_divisible_by(year, 400);
    
    (divisible_by_4 & !divisible_by_100) | (divisible_by_4 & divisible_by_400)
}

fn is_divisible_by(num: u64, div: u64) -> bool {
    num % div == 0
}