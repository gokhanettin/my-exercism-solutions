use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut map, (&k, v)| {
        v.iter().map(char::to_ascii_lowercase).for_each(|c| {
            map.insert(c, k);
        });
        map
    })
}
