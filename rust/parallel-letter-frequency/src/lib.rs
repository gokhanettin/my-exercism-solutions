use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel();
    (0..worker_count)
        .map(|i| {
            let input = input
                .iter()
                .skip(i)
                .step_by(worker_count)
                .flat_map(|s| s.chars())
                .collect::<Vec<_>>();
            let input = input.clone();
            let tx = tx.clone();
            thread::spawn(move || {
                let tally = input
                    .iter()
                    .map(|c| c.to_ascii_lowercase())
                    .filter(|c| c.is_alphabetic())
                    .fold(HashMap::new(), |mut acc, c| {
                        *acc.entry(c).or_insert(0) += 1;
                        acc
                    });
                tx.send(tally).unwrap();
            })
        })
        .for_each(|handle| handle.join().unwrap());

    rx.iter()
        .take(worker_count)
        .reduce(|mut tallies, tally| {
            for (k, v) in tally {
                *tallies.entry(k).or_insert(0) += v;
            }
            tallies
        })
        .unwrap_or_default()
}
