pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(ri, row)| {
            let max = row.iter().max().unwrap_or(&0);
            row.iter().enumerate().filter_map(move |(ci, &val)| {
                if val == *max {
                    Some((ri, ci, val))
                } else {
                    None
                }
            })
        })
        .filter_map(|(ri, ci, val)| {
            if input.iter().all(|row| row[ci] >= val) {
                Some((ri, ci))
            } else {
                None
            }
        })
        .collect()
}
