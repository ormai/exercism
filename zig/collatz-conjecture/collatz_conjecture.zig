pub const ComputationError = error{IllegalArgument};

pub fn steps(number: usize) ComputationError!usize {
    return switch (number) {
        0 => ComputationError.IllegalArgument,
        1 => 0,
        else => 1 + try steps(if (number % 2 == 0) number / 2 else 3 * number + 1),
    };
}
