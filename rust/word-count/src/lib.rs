use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| !(c.is_alphanumeric() || c == '\''))
        .fold(HashMap::new(), |mut acc, word| {
            if !word.is_empty() {
                let word = word.trim_matches('\'').to_ascii_lowercase();
                *acc.entry(word).or_insert(0) += 1;
            }
            acc
        })
}
