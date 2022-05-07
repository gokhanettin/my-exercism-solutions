pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
              Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        1 => format!(
            "{0} bottle of beer on the wall, {0} bottle of beer.\n\
             Take it down and pass it around, no more bottles of beer on the wall.\n",
            n
        ),
        2 => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\n\
             Take one down and pass it around, {1} bottle of beer on the wall.\n",
            n,
            n - 1
        ),
        3..=99 => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\n\
             Take one down and pass it around, {1} bottles of beer on the wall.\n",
            n,
            n - 1
        ),
        _ => panic!("More than 99 bottles!"),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
