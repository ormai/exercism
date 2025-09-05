const std = @import("std");

pub const Classification = enum {
    deficient,
    perfect,
    abundant,
};

/// Asserts that `n` is nonzero.
pub fn classify(n: u64) Classification {
    std.debug.assert(n > 0);

    var sum: u64 = 0;
    for (1..n) |f| {
        if (n % f == 0) {
            sum += f;
        }
    }

    return if (n == sum) Classification.perfect else if (n < sum) Classification.abundant else Classification.deficient;
}
