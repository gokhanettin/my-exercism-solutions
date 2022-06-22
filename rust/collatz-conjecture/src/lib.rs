pub fn collatz(n: u64) -> Option<u64> {
    let mut number = n;
    for i in 0.. {
        match number {
            0 => break,
            1 => return Some(i),
            even if even & 1 == 0 => number /= 2,
            _ => number = number.checked_mul(3)?.checked_add(1)?,

        }
    }
    None
}
