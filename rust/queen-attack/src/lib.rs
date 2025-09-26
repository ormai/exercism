#[derive(Debug)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(Self {
                rank: rank as u8,
                file: file as u8,
            })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.rank == other.pos.rank
            || self.pos.file == other.pos.file
            || self.pos.rank.abs_diff(other.pos.rank) == self.pos.file.abs_diff(other.pos.file)
    }
}
