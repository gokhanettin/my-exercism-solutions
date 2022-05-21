use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let all_letters: HashSet<_> = ('a'..='z').collect();
    let some_letters: HashSet<_> = sentence
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect();
    some_letters == all_letters
}
