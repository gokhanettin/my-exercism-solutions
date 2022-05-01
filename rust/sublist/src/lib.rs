#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if first_list.len() > second_list.len() {
        for i in 0..=first_list.len() - second_list.len() {
            if &first_list[i..i + second_list.len()] == second_list {
                return Comparison::Superlist;
            }
        }
    } else if first_list.len() < second_list.len() {
        for i in 0..=second_list.len() - first_list.len() {
            if &second_list[i..i + first_list.len()] == first_list {
                return Comparison::Sublist;
            }
        }
    }
    Comparison::Unequal
}
