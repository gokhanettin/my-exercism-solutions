use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        String::new()
    } else {
        list.iter()
            .zip(list.iter().skip(1))
            .map(|(x, y)| format!("For want of a {} the {} was lost.", x, y))
            .chain(once(format!(
                "And all for the want of a {}.",
                list.first().unwrap()
            )))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
