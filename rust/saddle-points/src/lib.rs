pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let candidates = input
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut candidates, (row, vec)| {
            if let Some(&max) = vec.iter().max() {
                for col in vec
                    .iter()
                    .enumerate()
                    .filter(|(_, &val)| val == max)
                    .map(|(col, _)| col)
                {
                    candidates.push((row, col, max));
                }
            }
            candidates
        });

    candidates
        .into_iter()
        .filter(|&(_, col, max)| {
            if let Some(min) = input.iter().map(|vec| vec[col]).min() {
                min == max
            } else {
                false
            }
        })
        .map(|(row, col, _)| (row, col))
        .collect()
}
