pub fn abbreviate(phrase: &str) -> String {
    let mut answer = String::new();

    for (index, char) in phrase.chars().enumerate() {
        if char.is_alphabetic()
            && (answer.is_empty()
                || phrase.chars().nth(index - 1).unwrap() == ' '
                || phrase.chars().nth(index - 1).unwrap() == '-'
                || (char.is_uppercase() && !phrase.chars().nth(index - 1).unwrap().is_uppercase()))
        {
            answer.push(char);
        }
    }

    answer.to_uppercase()
}
