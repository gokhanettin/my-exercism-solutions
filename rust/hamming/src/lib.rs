/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        let distance = s1
            .bytes()
            .zip(s2.bytes())
            .filter(|(l1, l2)| l1 != l2)
            .count();
        Some(distance)
    }
}
