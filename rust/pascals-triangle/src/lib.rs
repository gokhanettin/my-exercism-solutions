use std::iter;

pub struct PascalsTriangle {
    row_count: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count: row_count as usize,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.row_count == 0 {
            Vec::new()
        } else {
            let mut initial = Vec::with_capacity(self.row_count);
            initial.push(vec![1]);
            (1..self.row_count).fold(initial, |mut acc, _| {
                let prev = acc.last().unwrap();
                let curr: Vec<_> = iter::once(&0)
                    .chain(prev.iter())
                    .zip(prev.iter().chain(iter::once(&0)))
                    .map(|(x, y)| x + y)
                    .collect();
                acc.push(curr);
                acc
            })
        }
    }
}
