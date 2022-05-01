/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .rev()
        .try_fold((0, 0), |(n, acc), c| {
            c.to_digit(10)
                .map(|digit| if n % 2 == 1 { digit * 2 } else { digit })
                .map(|digit| if digit > 9 { digit - 9 } else { digit })
                .map(|digit| (n + 1, acc + digit))
        })
        .map_or(false, |(n, acc)| n > 1 && acc % 10 == 0)
}
