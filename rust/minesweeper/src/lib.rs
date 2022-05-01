use std::str;

#[rustfmt::skip]
const NEIGHBOR_OFFSETS: &[(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let nrows = minefield.len() as i32;
    (0..nrows)
        .map(|row| {
            let ncols = minefield[row as usize].len() as i32;
            (0..ncols)
                .map(
                    |col| match minefield[row as usize].as_bytes()[col as usize] {
                        b'*' => '*',
                        _ => {
                            let count = NEIGHBOR_OFFSETS
                                .iter()
                                .map(|(row_offset, col_offset)| {
                                    (row + row_offset, col + col_offset)
                                })
                                .filter(|&(row, col)| {
                                    (0 <= row && row < nrows) && (0 <= col && col < ncols)
                                })
                                .filter(|&(row, col)| {
                                    minefield[row as usize].as_bytes()[col as usize] == b'*'
                                })
                                .count() as u8;
                            if count == 0 {
                                ' '
                            } else {
                                (count + b'0') as char
                            }
                        }
                    },
                )
                .collect()
        })
        .collect()
}
