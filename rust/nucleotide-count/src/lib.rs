use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let map = nucleotide_counts(dna)?;
    map.get(&nucleotide).ok_or(nucleotide).copied()
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = [('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .iter()
        .copied()
        .collect();

    for ch in dna.chars() {
        match ch {
            'A' | 'C' | 'G' | 'T' => *map.entry(ch).or_insert(0) += 1,
            _ => return Err(ch),
        }
    }

    Ok(map)
}
