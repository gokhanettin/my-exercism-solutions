use std::collections::BTreeMap;

pub struct School {
    map: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            map: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.map
            .entry(grade)
            .or_insert(vec![])
            .push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.map.keys().cloned().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.map.get(&grade).map(|students| {
            let mut clone = students.clone();
            clone.sort_unstable();
            clone
        })
    }
}
