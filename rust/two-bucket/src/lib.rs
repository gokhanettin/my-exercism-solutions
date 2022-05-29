use std::cmp;
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut queue = VecDeque::new();
    let mut set = HashSet::new();

    match start_bucket {
        Bucket::One => {
            queue.push_back(((capacity_1, 0), 1));
            set.insert((0, capacity_2));
        }
        Bucket::Two => {
            queue.push_back(((0, capacity_2), 1));
            set.insert((capacity_1, 0));
        }
    }

    while let Some(((bucket_1, bucket_2), moves)) = queue.pop_front() {
        if bucket_1 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: bucket_2,
            });
        } else if bucket_2 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: bucket_1,
            });
        } else {
            let pour_1 = cmp::min(capacity_2 - bucket_2, bucket_1);
            let pour_2 = cmp::min(capacity_1 - bucket_1, bucket_2);
            let states = [
                // Empty
                (0, bucket_2),
                (bucket_1, 0),
                // Fill
                (capacity_1, bucket_2),
                (bucket_1, capacity_2),
                // Pour
                (bucket_1 - pour_1, bucket_2 + pour_1),
                (bucket_1 + pour_2, bucket_2 - pour_2),
            ];

            for state in states {
                if set.contains(&state) {
                    continue;
                }
                set.insert(state);
                queue.push_back((state, moves + 1));
            }
        }
    }
    None
}
