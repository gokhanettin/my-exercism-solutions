use std::ops;

pub struct Triangle {
    num_different_sides: usize,
}

impl Triangle {
    pub fn build<T>(sides: [T; 3]) -> Option<Self>
    where
        T: Copy + PartialOrd + ops::Add<Output = T>,
    {
        let mut sides = sides.to_vec();

        sides.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        if sides[2] < sides[0] + sides[1] {
            sides.dedup();
            Some(Triangle {
                num_different_sides: sides.len(),
            })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.num_different_sides == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.num_different_sides == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.num_different_sides < 3
    }
}
