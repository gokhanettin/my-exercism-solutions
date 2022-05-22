fn encode_primitive(n: u64) -> Option<&'static str> {
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

fn encode_chunk(n: u64) -> String {
    match encode_primitive(n) {
        Some(say) => say.to_string(),
        None => match n {
            20..=99 => {
                let (x, y) = ((n / 10) * 10, n % 10);
                format!(
                    "{}-{}",
                    encode_primitive(x).unwrap(),
                    encode_primitive(y).unwrap()
                )
            }
            100..=999 => match (n / 100, n % 100) {
                (x, 0) => format!("{} hundred", encode_primitive(x).unwrap()),
                (x, y) => format!(
                    "{} hundred {}",
                    encode_primitive(x).unwrap(),
                    encode_chunk(y),
                ),
            },
            _ => panic!("{} not a chunk", n),
        },
    }
}

pub fn encode(n: u64) -> String {
    if n < 1000 {
        return encode_chunk(n);
    }

    let mut chunks = Vec::new();
    let mut number = n;
    while number > 0 {
        chunks.push(number % 1000);
        number /= 1000;
    }
    let scales = vec![
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];

    chunks
        .iter()
        .rev()
        .zip(scales[..chunks.len()].iter().rev())
        .fold(String::new(), |mut acc, (&chunk, &scale)| {
            if chunk > 0 {
                acc.push_str(&format!("{} {} ", encode_chunk(chunk), scale));
            }
            acc
        })
        .trim_end()
        .to_string()
}
