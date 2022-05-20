use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut power: i64 = -1;
    let mut prev = ' ';
    let mut letters = HashMap::new();
    for c in input.chars().rev() {
        match c {
            'A'..='Z' => {
                let (coeff, _) = letters.entry(c).or_insert((0, false));
                *coeff += power;
                power *= 10;
                prev = c;
            }
            '=' | '+' => {
                let (_, leading) = letters.entry(prev).or_insert((0, false));
                power = 1;
                *leading = true;
            }
            _ => {}
        }
    }
    let (_, leading) = letters.entry(prev).or_insert((0, false));
    *leading = true;

    let letters: Vec<_> = letters
        .into_iter()
        .map(|(letter, (coeff, leading))| (letter, coeff, leading))
        .collect();
    let digits: HashSet<i64> = (0i64..10).collect();
    let mut stack = vec![(Vec::with_capacity(letters.len()), digits)];

    while let Some((path, digits)) = stack.pop() {
        if path.len() == letters.len() {
            let sum: i64 = letters
                .iter()
                .map(|(_, coeff, _)| coeff)
                .zip(path.iter())
                .map(|(coeff, digit)| coeff * (*digit as i64))
                .sum();
            if sum == 0 {
                return Some(
                    letters
                        .iter()
                        .map(|&(letter, _, _)| letter)
                        .zip(path.into_iter())
                        .collect(),
                );
            }
            continue;
        }
        for digit in &digits {
            if *digit == 0 && letters[path.len()].2 {
                continue;
            }
            let mut path = path.clone();
            let mut digits = digits.clone();
            path.push(*digit as u8);
            digits.remove(digit);
            stack.push((path, digits))
        }
    }
    None
}
