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
        if second_list.is_empty() {
            return Comparison::Superlist;
        }
        for slice in first_list.windows(second_list.len()){
            if slice == second_list {
                return Comparison::Superlist;
            }
        }
    } else if first_list.len() < second_list.len()  {
        if first_list.is_empty() {
            return Comparison::Sublist;
        }
        for slice in second_list.windows(first_list.len()) {
            if slice == first_list {
                return Comparison::Sublist;
            }
        }
    }
    Comparison::Unequal
}
