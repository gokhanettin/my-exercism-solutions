#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let Self {
            position: ChessPosition { rank, file },
        } = self;
        let Self {
            position:
                ChessPosition {
                    rank: other_rank,
                    file: other_file,
                },
        } = other;

        match (rank - other_rank, file - other_file) {
            (0, _) => true,
            (_, 0) => true,
            (x, y) if x == y || x == -y => true,
            _ => false,
        }
    }
}
