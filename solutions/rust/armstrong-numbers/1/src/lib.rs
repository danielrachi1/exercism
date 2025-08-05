pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    let number_of_digits = digits.len() as u32;

    digits.into_iter()
        .map(|digit| digit.pow(number_of_digits))
        .sum::<u32>() == num
}

fn digits(mut n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    if n == 0 {
        digits.push(0);
    } else {
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
    }
    digits
}
