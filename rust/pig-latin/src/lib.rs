pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            if word.starts_with(&['a', 'e', 'i', 'u', 'o'])
                || word.starts_with("xr")
                || word.starts_with("yt")
            {
                format!("{}{}", word, "ay")
            } else {
                for i in 0..word.len() {
                    if word[i..].starts_with("qu") {
                        return format!("{}{}{}", &word[i + 2..], &word[..i + 2], "ay");
                    }
                    if i != 0 && word[i..].starts_with(&['a', 'e', 'i', 'u', 'o', 'y']) {
                        return format!("{}{}{}", &word[i..], &word[..i], "ay");
                    }
                }
                return word.to_string();
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
