/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut numbers = vec![];
    for (i, b) in isbn.bytes().rev().enumerate() {
        match (i, b) {
            (_, b'-') => continue,
            (0, b'X') => numbers.push(10),
            (_, b'0'..=b'9') => numbers.push((b - b'0') as usize),
            _ => return false,
        }
    }

    if numbers.len() != 10 {
        false
    } else {
        let sum: usize = numbers.iter().enumerate().map(|(i, n)| (i + 1) * n).sum();
        sum % 11 == 0
    }
}
