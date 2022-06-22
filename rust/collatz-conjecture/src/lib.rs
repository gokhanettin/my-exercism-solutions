pub fn collatz(n: u64) -> Option<u64> {
    let mut number = n;
    for i in 0.. {
        if number == 0 {
            return None;
        } else if number == 1 {
            return Some(i);
        } else {
            number = match number % 2 {
                0 => number / 2,
                _ => number.checked_mul(3)?.checked_add(1)?,
            };
        }
    }
    unreachable!()
}
