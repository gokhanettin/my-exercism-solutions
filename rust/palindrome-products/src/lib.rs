/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if value < 10 {
            Some(Palindrome(value))
        } else {
            let (mut rev, mut num) = (0, value);
            while num > 0 {
                rev = 10 * rev + num % 10;
                num /= 10;
            }
            if rev == value {
                Some(Palindrome(value))
            } else {
                None
            }
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let palindromes: Vec<_> = (min..=max)
        .flat_map(|x| (x..=max).filter_map(move |y| Palindrome::new(x * y)))
        .collect();

    println!("{:?}", palindromes);
    let min = palindromes
        .iter()
        .min_by_key(|palindrome| palindrome.0)
        .cloned();
    let max = palindromes
        .iter()
        .max_by_key(|palindrome| palindrome.0)
        .cloned();
    min.zip(max)
}
