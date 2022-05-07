pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::new();
    for c in string.chars() {
        match c {
            '[' | '(' | '{' => brackets.push(c),
            ']' | ')' | '}' => {
                if let Some(b) = brackets.pop() {
                    match (b, c) {
                        ('[', ']') => continue,
                        ('(', ')') => continue,
                        ('{', '}') => continue,
                        (_, _) => return false,
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    brackets.is_empty()
}
