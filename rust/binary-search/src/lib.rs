pub fn find<T, A>(array: A, key: T) -> Option<usize>
where
    T: PartialOrd,
    A: AsRef<[T]>,
{
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let mut slice = array;
    let mut position = 0;

    while slice.len() > 1 {
        let half = slice.len() / 2;
        if slice[half] > key {
            slice = &slice[..half];
        } else {
            slice = &slice[half..];
            position += half;
        }
    }

    if array[position] == key {
        Some(position)
    } else {
        None
    }
}
