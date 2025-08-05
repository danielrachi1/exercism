use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    
    for possible_anagram in possible_anagrams {
        if is_anagram(word, possible_anagram) {
            anagrams.insert(*possible_anagram);
        }
    }

    anagrams
}

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    if word.to_lowercase() == possible_anagram.to_lowercase() {
        return false
    }
    
    let mut char_count = HashMap::new();

    word.to_lowercase().chars().for_each(|character| {
        char_count
            .entry(character)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    possible_anagram.to_lowercase().chars().for_each(|character| {
        char_count
            .entry(character)
            .and_modify(|count| *count -= 1)
            .or_insert(-1);
    });

    char_count.iter().all(|(_, count)| *count == 0)
}
