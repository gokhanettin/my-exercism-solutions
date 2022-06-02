use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::RefCell;

const MAX_ROBOT_COUNT: u32 = 26 * 26 * 10 * 10 * 10;

#[derive(Default)]
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut robot: Robot = Default::default();
        robot.reset_name();
        robot
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        thread_local!(static NUMBERS: RefCell<UniqueIter> = RefCell::new(unique_iter(MAX_ROBOT_COUNT)));
        NUMBERS.with(|numbers| {
            let mut id = numbers.borrow_mut().next().unwrap();
            self.name.clear();
            self.name.push((b'A' + (id % 26) as u8) as char);
            id /= 26;
            self.name.push((b'A' + (id % 26) as u8) as char);
            id /= 26;
            self.name.push((b'0' + (id % 10) as u8) as char);
            id /= 10;
            self.name.push((b'0' + (id % 10) as u8) as char);
            id /= 10;
            self.name.push((b'0' + (id % 10) as u8) as char);
        });
    }
}

type UniqueIter = std::vec::IntoIter<u32>;

fn unique_iter(n: u32) -> UniqueIter {
    let mut vec: Vec<u32> = (0..n).collect();
    let mut rng = thread_rng();
    vec.shuffle(&mut rng);
    vec.into_iter()
}
