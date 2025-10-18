#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let (caps, mut state, bckts) = match start_bucket {
        Bucket::One => (
            (capacity_1, capacity_2),
            (capacity_1, 0),
            (Bucket::One, Bucket::Two),
        ),
        Bucket::Two => (
            (capacity_2, capacity_1),
            (capacity_2, 0),
            (Bucket::Two, Bucket::One),
        ),
    };
    let mut moves = 1;
    while state.0 != goal && state.1 != goal {
        match state {
            (_, _) if caps.1 == goal => state.1 = goal,
            (0, _) => state.0 = caps.0,
            (l1, l2) if l1 < caps.0 && l2 == caps.1 => state.1 = 0,
            (l1, l2) if l1 <= caps.0 && l2 < caps.1 => {
                state.0 = l1 - l1.min(caps.1 - l2);
                state.1 = l2 + l1.min(caps.1 - l2);
            }
            _ => return None,
        }
        moves += 1;
    }
    Some(BucketStats {
        goal_bucket: if state.1 == goal { bckts.1 } else { bckts.0 },
        other_bucket: if state.1 == goal { state.0 } else { state.1 },
        moves,
    })
}
