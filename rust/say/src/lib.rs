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
    let mut number = n;
    let mut encoded = String::new();
    if let Some(say) = encode_primitive(number) {
        encoded.push_str(say);
    } else {
        if n >= 100 {
            let primitive = encode_primitive(number / 100).unwrap();
            encoded.push_str(primitive);
            encoded.push_str(" hundred");
            number %= 100;
            if number > 0 {
                encoded.push(' ');
            }
        }
        if let Some(say) = encode_primitive(number) {
            if number > 0 {
                encoded.push_str(say);
            }
        } else {
            let primitive = encode_primitive((number / 10) * 10).unwrap();
            encoded.push_str(primitive);
            encoded.push('-');
            let primitive = encode_primitive(number % 10).unwrap();
            encoded.push_str(primitive);
        }
    }
    encoded
}

pub fn encode(n: u64) -> String {
    if n < 1000 {
        return encode_chunk(n);
    }

    let mut number = n;
    let mut encoded = String::new();
    let mut chunks = Vec::new();
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

    let chunk_count = chunks.len();
    for (index, chunk) in chunks.into_iter().rev().enumerate() {
        if chunk > 0 {
            if !encoded.is_empty() {
                encoded.push(' ');
            }
            encoded.push_str(&encode_chunk(chunk));
            let i = chunk_count - index - 1;
            if i > 0 {
                encoded.push(' ');
            }
            encoded.push_str(scales[i]);
        }
    }
    encoded
}
