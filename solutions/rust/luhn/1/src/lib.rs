pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|c| !(c.is_digit(10) | c.is_whitespace())) {
        return false;
    }
    
    let digits: Vec<u32> = code.chars().filter_map(|c| c.to_digit(10)).collect();

    if digits.len() <= 1 {
        return false;
    }

    let luhn_sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(index, &digit_val)| {
            if index % 2 == 1 {
                let mut doubled_digit = digit_val * 2;
                if doubled_digit > 9 {
                    doubled_digit -= 9;
                }
                doubled_digit
            } else {
                digit_val
            }
        })
        .sum();

    luhn_sum % 10 == 0
}
