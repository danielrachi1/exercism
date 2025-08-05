fn num_to_word(n: u32) -> &'static str {
    match n {
        0 => "no",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => "many", // Fallback for numbers > 10
    }
}

pub fn verse(n: u32) -> String {
    match n {
        0 => String::new(), // No verse for zero bottles
        1 => {
            "One green bottle hanging on the wall,\n\
             One green bottle hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be no green bottles hanging on the wall."
                .to_string()
        }
        2 => {
            "Two green bottles hanging on the wall,\n\
             Two green bottles hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be one green bottle hanging on the wall."
                .to_string()
        }
        _ => {
            let current_word_capitalized = {
                let word = num_to_word(n);
                let mut c = word.chars();
                c.next().unwrap().to_uppercase().to_string() + c.as_str()
            };

            format!(
                "{} green bottles hanging on the wall,\n\
                 {} green bottles hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be {} green bottles hanging on the wall.",
                current_word_capitalized,
                current_word_capitalized,
                num_to_word(n - 1)
            )
        }
    }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let verses: Vec<String> = (start_bottles.saturating_sub(take_down - 1)..=start_bottles)
        .rev()
        .map(verse)
        .collect();

    verses.join("\n\n")
}