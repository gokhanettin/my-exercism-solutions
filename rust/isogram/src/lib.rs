use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate
        .bytes()
        .filter(|&b| b != b' ' && b != b'-')
        .map(|b| b.to_ascii_lowercase())
        .all(|b| set.insert(b))
}
