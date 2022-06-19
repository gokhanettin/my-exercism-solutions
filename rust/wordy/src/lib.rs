pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    let words: Vec<_> = command
        .trim_end_matches("?")
        .split_ascii_whitespace()
        .collect();

    match words.as_slice() {
        &["What", "is", lhs, ref rest @ ..] => {
            let lhs = lhs.parse::<i32>().ok()?;
            op(lhs, rest)
        }
        _ => None,
    }
}

fn op(lhs: i32, rest: &[&str]) -> Option<i32> {
    match rest {
        &["plus", rhs, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            op(lhs + rhs, rest)
        }
        &["minus", rhs, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            op(lhs - rhs, rest)
        }
        &["multiplied", "by", rhs, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            op(lhs * rhs, rest)
        }
        &["divided", "by", rhs, ref rest @ ..] => {
            let rhs = rhs.parse::<i32>().ok()?;
            op(lhs / rhs, rest)
        }
        &["raised", "to", "the", rhs, "power", ref rest @ ..] => {
            let rhs = rhs[..rhs.len() - 2].parse::<u32>().ok()?;
            op(lhs.pow(rhs) , rest)
        }
        &[] => Some(lhs),
        &[..] => None,
    }
}
