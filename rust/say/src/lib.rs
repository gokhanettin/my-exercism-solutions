const SCALES: [(u64, &str); 7] = [
    (10u64.pow(2), "hundred"),
    (10u64.pow(3), "thousand"),
    (10u64.pow(6), "million"),
    (10u64.pow(9), "billion"),
    (10u64.pow(12), "trillion"),
    (10u64.pow(15), "quadrillion"),
    (10u64.pow(18), "quintillion"),
];

fn encode_basic(n: u64) -> Option<&'static str> {
    match n {
        0 => Some("zero"),
        1 => Some("one"),
        2 => Some("two"),
        3 => Some("three"),
        4 => Some("four"),
        5 => Some("five"),
        6 => Some("six"),
        7 => Some("seven"),
        8 => Some("eight"),
        9 => Some("nine"),
        10 => Some("ten"),
        11 => Some("eleven"),
        12 => Some("twelve"),
        13 => Some("thirteen"),
        14 => Some("fourteen"),
        15 => Some("fifteen"),
        16 => Some("sixteen"),
        17 => Some("seventeen"),
        18 => Some("eighteen"),
        19 => Some("nineteen"),
        20 => Some("twenty"),
        30 => Some("thirty"),
        40 => Some("forty"),
        50 => Some("fifty"),
        60 => Some("sixty"),
        70 => Some("seventy"),
        80 => Some("eighty"),
        90 => Some("ninety"),
        _ => None,
    }
}

pub fn encode(n: u64) -> String {
    if n < 100 {
        match encode_basic(n) {
            Some(say) => say.to_string(),
            None => {
                let (ten, one) = ((n / 10) * 10, n % 10);
                format!(
                    "{}-{}",
                    encode_basic(ten).unwrap(),
                    encode_basic(one).unwrap()
                )
            }
        }
    } else {
        for (pow, name) in SCALES.into_iter().rev() {
            match (n / pow, n % pow) {
                (0, _) => continue,
                (quo, 0) => return format!("{} {}", encode(quo), name),
                (quo, rem) => return format!("{} {} {}", encode(quo), name, encode(rem)),
            }
        }
        unreachable!();
    }
}
