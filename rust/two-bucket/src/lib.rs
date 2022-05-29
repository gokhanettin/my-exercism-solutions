use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
    let state = match start_bucket {
        Bucket::One => State::new(capacity_1, 0, 1, capacity_1, capacity_2, *start_bucket),
        Bucket::Two => State::new(0, capacity_2, 1, capacity_1, capacity_2, *start_bucket),
    }
    .unwrap();

    set.insert(Rc::clone(&state));
    queue.push_back(state);

    while let Some(state) = queue.pop_front() {
        let (bucket_1, bucket_2) = state.buckets;
        if bucket_1 == goal {
            return Some(BucketStats {
                moves: state.moves,
                goal_bucket: Bucket::One,
                other_bucket: bucket_2,
            });
        } else if bucket_2 == goal {
            return Some(BucketStats {
                moves: state.moves,
                goal_bucket: Bucket::Two,
                other_bucket: bucket_1,
            });
        } else {
            for function in FUNCTIONS {
                if let Some(state) = function(&state) {
                    if !set.contains(&state) {
                        set.insert(Rc::clone(&state));
                        queue.push_back(state);
                    }
                }
            }
        }
    }
    None
}

const FUNCTIONS: [for<'r> fn(&'r State) -> Option<Rc<State>>; 6] = [
    State::fill_1,
    State::fill_2,
    State::empty_1,
    State::empty_2,
    State::pour_1,
    State::pour_2,
];

#[derive(Debug)]
struct State {
    moves: u8,
    start_bucket: Bucket,
    buckets: (u8, u8),
    capacities: (u8, u8),
}

impl State {
    fn new(
        bucket_1: u8,
        bucket_2: u8,
        moves: u8,
        capacity_1: u8,
        capacity_2: u8,
        start_bucket: Bucket,
    ) -> Option<Rc<State>> {
        if bucket_1 > capacity_1 || bucket_2 > capacity_2 {
            None
        } else if start_bucket == Bucket::One && bucket_2 == capacity_2 && bucket_1 == 0 {
            None
        } else if start_bucket == Bucket::Two && bucket_1 == capacity_1 && bucket_2 == 0 {
            None
        } else {
            Some(Rc::new(State {
                moves,
                start_bucket,
                buckets: (bucket_1, bucket_2),
                capacities: (capacity_1, capacity_2),
            }))
        }
    }

    fn fill_1(&self) -> Option<Rc<State>> {
        State::new(
            self.capacities.0,
            self.buckets.1,
            self.moves + 1,
            self.capacities.0,
            self.capacities.1,
            self.start_bucket,
        )
    }

    fn fill_2(&self) -> Option<Rc<State>> {
        State::new(
            self.buckets.0,
            self.capacities.1,
            self.moves + 1,
            self.capacities.0,
            self.capacities.1,
            self.start_bucket,
        )
    }

    fn empty_1(&self) -> Option<Rc<State>> {
        State::new(
            0,
            self.buckets.1,
            self.moves + 1,
            self.capacities.0,
            self.capacities.1,
            self.start_bucket,
        )
    }

    fn empty_2(&self) -> Option<Rc<State>> {
        State::new(
            self.buckets.0,
            0,
            self.moves + 1,
            self.capacities.0,
            self.capacities.1,
            self.start_bucket,
        )
    }

    fn pour_1(&self) -> Option<Rc<State>> {
        let pour = cmp::min(self.capacities.1 - self.buckets.1, self.buckets.0);
        State::new(
            self.buckets.0 - pour,
            self.buckets.1 + pour,
            self.moves + 1,
            self.capacities.0,
            self.capacities.1,
            self.start_bucket,
        )
    }
    fn pour_2(&self) -> Option<Rc<State>> {
        let pour = cmp::min(self.capacities.0 - self.buckets.0, self.buckets.1);
        State::new(
            self.buckets.0 + pour,
            self.buckets.1 - pour,
            self.moves + 1,
            self.capacities.0,
            self.capacities.1,
            self.start_bucket,
        )
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buckets.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.buckets == other.buckets
    }
}

impl Eq for State {}
