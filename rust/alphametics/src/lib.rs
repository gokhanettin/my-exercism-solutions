use std::collections::HashMap;
use std::collections::HashSet;
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
            stack: vec![(Vec::new(), elements.iter())],
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
    let unknowns: Vec<&str> = input
        .split(&[' ', '+', '='])
        .filter(|s| !s.is_empty())
        .collect();
    let letters = unknowns
        .iter()
        .flat_map(|s| s.bytes())
        .fold(HashSet::new(), |mut acc, c| {
            acc.insert(c);
            acc
        });

    let first_letters: Vec<_> = unknowns
        .iter()
        .map(|s| s.as_bytes().first().unwrap_or(&b'0'))
        .collect();

    let digits = b"1234567890";
    let permutations = Permutation::new(digits, letters.len());

    for permutation in permutations {
        let dict: HashMap<u8, u8> = letters
            .iter()
            .copied()
            .zip(permutation.into_iter().copied())
            .collect();

        if first_letters
            .iter()
            .any(|&b| dict.get(&b).map_or(false, |&digit| digit == b'0'))
        {
            continue;
        }
        let guesses: Vec<Vec<_>> = unknowns
            .iter()
            .filter_map(|s| {
                s.bytes()
                    .map(|b| dict.get(&b).copied())
                    .collect::<Option<Vec<_>>>()
            })
            .collect();

        if guesses.len() != unknowns.len() {
            continue;
        }

        let numbers: Vec<u64> = guesses
            .iter()
            .flat_map(|v| String::from_utf8_lossy(v).parse::<u64>())
            .collect();

        if numbers.iter().sum::<u64>() == 2 * numbers.last().unwrap() {
            return Some(dict.iter().fold(HashMap::new(), |mut acc, (&k, &v)| {
                acc.insert(k as char, v - b'0');
                acc
            }));
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
