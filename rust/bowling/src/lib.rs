use Error::*;

const MAX_FRAMES: usize = 10;
const PINS_PER_FRAME: u16 = 10;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    rolls: Vec<u16>,
    bonus: Vec<u16>,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl Frame {
    fn new() -> Self {
        Self {
            rolls: Vec::new(),
            bonus: Vec::new(),
        }
    }

    fn rolls_score(&self) -> u16 {
        self.rolls.iter().sum()
    }

    fn bonus_score(&self) -> u16 {
        self.bonus.iter().sum()
    }

    fn score(&self) -> u16 {
        self.rolls_score() + self.bonus_score()
    }

    fn is_strike(&self) -> bool {
        self.rolls.len() == 1 && self.rolls_score() == PINS_PER_FRAME
    }

    fn is_spare(&self) -> bool {
        self.rolls.len() == 2 && self.rolls_score() == PINS_PER_FRAME
    }

    fn is_open(&self) -> bool {
        self.rolls.len() == 2 && self.rolls_score() < PINS_PER_FRAME
    }

    fn bonus_done(&self) -> bool {
        (self.is_strike() && self.bonus.len() == 2) || (self.is_spare() && self.bonus.len() == 1)
    }

    fn rolls_done(&self) -> bool {
        self.rolls.len() == 2 || self.is_strike()
    }

    fn is_valid(&self) -> bool {
        self.rolls_valid() && self.bonus_valid()
    }

    fn rolls_valid(&self) -> bool {
        self.rolls_score() <= PINS_PER_FRAME
    }

    fn bonus_valid(&self) -> bool {
        if self.is_open() || !self.bonus_done() {
            return true;
        }

        let bonus_score = self.bonus_score();

        if self.is_spare() {
            return bonus_score <= PINS_PER_FRAME;
        }

        if let Some(&first) = self.bonus.first() {
            if first == PINS_PER_FRAME {
                bonus_score <= 2 * PINS_PER_FRAME
            } else {
                bonus_score <= PINS_PER_FRAME
            }
        } else {
            // self.is_open() || !self.bonus_done() guarantees this will never happen
            unreachable!();
        }
    }

    fn is_complete(&self) -> bool {
        self.is_open() || self.bonus_done()
    }

    fn add_roll(&mut self, roll: u16) {
        if !self.is_complete() {
            if self.is_spare() || self.is_strike() {
                self.bonus.push(roll);
            } else {
                self.rolls.push(roll)
            }
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![Frame::new()],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > PINS_PER_FRAME {
            return Err(NotEnoughPinsLeft);
        }

        if self.score().is_some() {
            return Err(GameComplete);
        }

        for frame in self.frames.iter_mut() {
            frame.add_roll(pins);
            if !frame.is_valid() {
                return Err(NotEnoughPinsLeft);
            }
        }

        if self.frames.last().unwrap().rolls_done() && self.frames.len() < MAX_FRAMES {
            self.frames.push(Frame::new());
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_done() {
            Some(self.frames.iter().fold(0, |acc, f| acc + f.score()))
        } else {
            None
        }
    }

    pub fn is_done(&self) -> bool {
        self.frames.len() == MAX_FRAMES && self.frames.last().unwrap().is_complete()
    }
}
