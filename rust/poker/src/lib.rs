use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut scores: Vec<_> = hands
        .iter()
        .map(|hand| evaluate(hand))
        .enumerate()
        .collect();
    scores.sort_by_key(|(_, eval)| eval.clone());
    let winner = scores.last().unwrap();
    scores
        .iter()
        .rev()
        .take_while(|score| winner.1 == score.1)
        .map(|score| hands[score.0])
        .collect::<Vec<_>>()
}

fn evaluate(hand: &str) -> (Vec<i32>, Vec<i32>) {
    const RANKS: &[u8; 13] = b"23456789TJQKA";
    let (rank_counts, suits) = hand.split(' ').fold(
        (HashMap::new(), HashSet::new()),
        |(mut rank_counts, mut suits), card| {
            let card = card.as_bytes();
            let suit = card.last().unwrap();
            let rank = RANKS
                .iter()
                .position(|r| {
                    let b = if card.len() == 3 {
                        &b'T'
                    } else {
                        card.first().unwrap()
                    };
                    r == b
                })
                .unwrap() as i32;
            *rank_counts.entry(rank).or_insert(0) += 1;
            suits.insert(suit);
            (rank_counts, suits)
        },
    );
    let mut rank_scores: Vec<_> = rank_counts.into_iter().collect();
    rank_scores.sort_by_key(|&(a, b)| Reverse((b, a)));
    let (mut ranks, mut scores): (Vec<_>, Vec<_>) = rank_scores.into_iter().unzip();
    if scores.len() == 5 {
        if &ranks[..2] == &[12, 3] {
            // Ace can start a straight
            ranks = vec![3, 2, 1, 0, -1];
        }
        let straight = ranks[0] - ranks[4] == 4;
        let flush = suits.len() == 1;
        scores = match (flush, straight) {
            (false, false) => vec![1],
            (false, true) => vec![3, 1, 1, 1],
            (true, false) => vec![3, 1, 1, 2],
            (true, true) => vec![5],
        };
    }
    (scores, ranks)
}
