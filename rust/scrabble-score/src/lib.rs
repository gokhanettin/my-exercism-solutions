/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let score_letter = |letter| match letter {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    };
    word.chars()
        .map(|c| score_letter(c.to_ascii_uppercase()))
        .sum()
}
