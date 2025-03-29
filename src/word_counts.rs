use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .map(|c| c.trim_matches('\''))
        .filter(|word| !word.is_empty())
        .fold(HashMap::new(), |mut map, word| {
            map.entry(word.to_lowercase())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            map
        })
}
