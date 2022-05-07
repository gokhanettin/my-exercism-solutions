struct Digits {
    num: u32,
    base: u32,
}

impl Digits {
    fn digits(num: u32, base: u32) -> Digits {
        Digits { num, base }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num < 1 {
            return None;
        } else {
            let digit = self.num % self.base;
            self.num /= self.base;
            Some(digit)
        }
    }
}

pub fn is_armstrong_number(num: u32) -> bool {
    let iter = Digits::digits(num, 10);
    let digits: Vec<_> = iter.collect();
    let p = digits.len() as u32;
    let sum: u32 = digits.iter().map(|d| d.pow(p)).sum();
    num == sum
}
