pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_ascii_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            let first = word.chars().map(|c| c.to_ascii_uppercase()).take(1);
            let rest = word
                .chars()
                .skip(1)
                .skip_while(|c| c.is_ascii_uppercase())
                .filter(|c| c.is_ascii_uppercase());
            first.chain(rest)
        })
        .collect()
}
