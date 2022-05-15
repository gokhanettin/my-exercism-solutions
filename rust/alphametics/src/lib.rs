use std::collections::HashMap;
use std::fmt;
use std::slice;

struct Permutation<'a, T>
where
    T: PartialEq + fmt::Debug,
{
    elements: &'a [T],
    size: usize,
    stack: Vec<State<'a, T>>,
}

type State<'a, T> = (Vec<&'a T>, slice::Iter<'a, T>);

impl<'a, T> Permutation<'a, T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn new(elements: &'a [T], size: usize) -> Self {
        Permutation {
            elements,
            size,
            stack: vec![(Vec::with_capacity(size), elements.iter())],
        }
    }
}

impl<'a, T> Iterator for Permutation<'a, T>
where
    T: PartialEq + std::fmt::Debug,
{
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((mut path, mut options)) = self.stack.pop() {
            path.pop();
            while let Some(option) = options.next() {
                if path.contains(&option) {
                    continue;
                }
                path.push(option);
                self.stack.push((path.clone(), options.clone()));
                if path.len() == self.size {
                    return Some(path);
                }
                let mut options = self.elements.iter();
                while let Some(option) = options.next() {
                    if path.contains(&option) {
                        continue;
                    }
                    path.push(option);
                    self.stack.push((path.clone(), options.clone()));
                    if path.len() == self.size {
                        return Some(path);
                    }
                }
            }
        }
        None
    }
}

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

    let digits = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let permutations = Permutation::new(digits, letters.len());
    'perm: for permutation in permutations {
        let mut sum = 0;
        for (&digit, (&letter, &(coeff, leading))) in permutation.iter().zip(letters.iter()) {
            match (digit, leading, letter, coeff) {
                (0, true, _, _) => continue 'perm,
                (digit, _, _, coeff) => {
                    sum += digit * coeff;
                }
            }
        }
        if sum == 0 {
            return Some(
                letters
                    .keys()
                    .copied()
                    .zip(permutation.iter().map(|&digit| *digit as u8))
                    .collect(),
            );
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_3_1() {
        let mut permutations: Permutation<i32> = Permutation::new(&[1, 2, 3], 1);
        assert_eq!(Some(vec![&1]), permutations.next());
        assert_eq!(Some(vec![&2]), permutations.next());
        assert_eq!(Some(vec![&3]), permutations.next());
        assert_eq!(None, permutations.next());
    }

    #[test]
    fn test_permutation_3_2() {
        let mut permutations: Permutation<i32> = Permutation::new(&[1, 2, 3], 2);
        assert_eq!(Some(vec![&1, &2]), permutations.next());
        assert_eq!(Some(vec![&1, &3]), permutations.next());
        assert_eq!(Some(vec![&2, &1]), permutations.next());
        assert_eq!(Some(vec![&2, &3]), permutations.next());
        assert_eq!(Some(vec![&3, &1]), permutations.next());
        assert_eq!(Some(vec![&3, &2]), permutations.next());
        assert_eq!(None, permutations.next());
    }

    #[test]
    fn test_permutation_3_3() {
        let mut permutations: Permutation<i32> = Permutation::new(&[1, 2, 3], 3);
        assert_eq!(Some(vec![&1, &2, &3]), permutations.next());
        assert_eq!(Some(vec![&1, &3, &2]), permutations.next());
        assert_eq!(Some(vec![&2, &1, &3]), permutations.next());
        assert_eq!(Some(vec![&2, &3, &1]), permutations.next());
        assert_eq!(Some(vec![&3, &1, &2]), permutations.next());
        assert_eq!(Some(vec![&3, &2, &1]), permutations.next());
        assert_eq!(None, permutations.next());
    }

    #[test]
    fn test_permutation_3_4() {
        let mut permutations: Permutation<i32> = Permutation::new(&[1, 2, 3], 4);
        assert_eq!(None, permutations.next());
    }
}
