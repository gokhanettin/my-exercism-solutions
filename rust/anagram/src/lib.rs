use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .fold(HashSet::new(), |mut hashset, &candidate| {
            if word == candidate {
                hashset
            } else {
                let mut word_lowercase: Vec<_> = word.to_lowercase().chars().collect();
                let mut candidate_lowercase: Vec<_> = candidate.to_lowercase().chars().collect();
                if word_lowercase != candidate_lowercase {
                    word_lowercase.sort_unstable();
                    candidate_lowercase.sort_unstable();
                    if word_lowercase == candidate_lowercase {
                        hashset.insert(candidate);
                    }
                }
                hashset
            }
        })
}
