pub fn raindrops(n: u32) -> String {
    let divisors = [
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong")
    ];
    
    let mut result = String::new();

    for (div, word) in divisors {
        if is_divisible(n,div) {
            result.push_str(word)
        }
    }

    if result.is_empty() {
        result = n.to_string();
    }

    result
}

fn is_divisible(n: u32, div: u32) -> bool {
    n % div == 0
}