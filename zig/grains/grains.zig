pub const ChessboardError = error{IndexOutOfBounds};

pub fn square(comptime index: usize) ChessboardError!u64 {
    return if (index > 0 and index <= 64) 1 << (index - 1) else ChessboardError.IndexOutOfBounds;
}

pub fn total() u64 {
    return ~@as(u64, 0);
}
