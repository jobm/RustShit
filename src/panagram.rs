use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>()
        .len()
        == 26
    // set.len() == 26
    // let s = sentence.to_lowercase();
    // ('a'..='z').all(|c| sentence.contains(c))
}
