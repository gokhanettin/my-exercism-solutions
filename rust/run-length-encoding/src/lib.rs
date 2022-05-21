pub fn encode(source: &str) -> String {
    let mut iter = source.chars().peekable();
    let mut encoded = String::new();
    while let Some(elem) = iter.next() {
        let mut count = 1;
        while let Some(&next) = iter.peek() {
            if elem == next {
                iter.next();
                count += 1;
            } else {
                break;
            }
        }
        if count == 1 {
            encoded.push(elem);
        } else {

            encoded.push_str(&format!("{}{}", count, elem));
        }
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut count = 0;
    let mut decoded = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            count = 10 * count + c.to_digit(10).unwrap();
        } else {
            if count == 0 {
                decoded.push(c);
            } else {
                for _ in 0..count {
                    decoded.push(c);
                }
                count = 0;
            }
        }
    }
    decoded
}
