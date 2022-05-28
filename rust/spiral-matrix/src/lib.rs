use std::iter;
use Direction::*;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {

    if size == 0 {
        Vec::new()
    } else {
        let size = size as usize;
        let mut directions = [Right, Down, Left, Up].iter().cycle();
        let points = iter::once((0, 0)).chain(
            iter::once(size - 1)
                .chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2)))
                .flat_map(|nsteps| iter::repeat(directions.next().unwrap()).take(nsteps))
                .scan((0, 0), |(row, col), direction| {
                    match direction {
                        Right => *col += 1,
                        Down => *row += 1,
                        Left => *col -= 1,
                        Up => *row -= 1,
                    }
                    Some((*row, *col))
                }),
        );

        let mut vec = vec![vec![0u32; size]; size];
        for (value, (row, col)) in points.enumerate() {
            vec[row][col] = value as u32 + 1;
        }
        vec
    }
}
