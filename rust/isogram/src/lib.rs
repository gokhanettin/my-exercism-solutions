use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    candidate
        .bytes()
        .filter(|&b| b != b' ' && b != b'-')
        .map(|b| b.to_ascii_lowercase())
        .fold(HashMap::new(), |mut acc, b| {
            *acc.entry(b).or_insert(0) += 1;
            acc
        })
        .iter()
        .all(|(_, &count)| count == 1)
}
