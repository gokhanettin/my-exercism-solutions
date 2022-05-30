// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use Direction::*;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        let d = match self.d {
            North => East,
            East => South,
            South => West,
            West => North,
        };

        Self { d, ..self }
    }

    pub fn turn_left(self) -> Self {
        let d = match self.d {
            North => West,
            East => North,
            South => East,
            West => South,
        };

        Self { d, ..self }
    }

    pub fn advance(self) -> Self {
        let (x, y) = match self {
            Self { x, y, d: North } => (x, y + 1),
            Self { x, y, d: East } => (x + 1, y),
            Self { x, y, d: South } => (x, y - 1),
            Self { x, y, d: West } => (x - 1, y),
        };

        Self { x, y, ..self }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for i in instructions.bytes() {
            robot = match i {
                b'L' => robot.turn_left(),
                b'R' => robot.turn_right(),
                b'A' => robot.advance(),
                _ => robot,
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        let &Self { x, y, .. } = self;
        (x, y)
    }

    pub fn direction(&self) -> &Direction {
        let Self { d, .. } = self;
        d
    }
}
