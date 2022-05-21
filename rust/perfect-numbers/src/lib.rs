#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else {
        let sum: u64 = (1..num).filter(|n| num % n == 0).sum();
        if sum == num {
            Some(Classification::Perfect)
        } else if sum > num {
            Some(Classification::Abundant)
        } else {
            Some(Classification::Deficient)
        }
    }
}
