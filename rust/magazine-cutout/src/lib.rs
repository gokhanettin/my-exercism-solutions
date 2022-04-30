use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_counts = HashMap::new();
    for word in magazine {
        let count = word_counts.entry(*word).or_insert(0u32);
        *count += 1;
    }
    for word in note {
        let count = word_counts.entry(*word).or_insert(0u32);
        if *count > 0u32 {
            *count -= 1;
        } else {
            return false;
        }
    }
    true
}
