#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => {
            if a.windows(n).any(|w| w == b) {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            if b.windows(m).any(|w| w == a) {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if a == b {
                Equal
            } else {
                Unequal
            }
        }
    }
}
