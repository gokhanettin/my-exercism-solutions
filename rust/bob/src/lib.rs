pub fn reply(message: &str) -> &str {
    let mut alphabetic = 0;
    let mut uppercase = 0;
    let mut nonspace = 0;
    let mut question = false;

    for c in message.chars().rev() {
        if c.is_whitespace() {
            continue;
        } else {
            nonspace += 1;
        }
        if nonspace == 1 && c == '?' {
            question = true;
        }
        if c.is_alphabetic() {
            alphabetic += 1;
        }
        if c.is_uppercase() {
            uppercase += 1;
        }
    }
    if uppercase == alphabetic && alphabetic != 0 {
        if question {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if question {
        "Sure."
    } else if nonspace == 0 {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
