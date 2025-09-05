const std = @import("std");

pub fn isArmstrongNumber(num: u128) bool {
    var cursor: usize = 0;
    var digits: [64]u4 = undefined;
    var n = num;
    while (n > 0) : (n /= 10) {
        digits[cursor] = @intCast(n % 10);
        cursor += 1;
    }

    var sum: u128 = 0;
    for (0..cursor) |i| {
        sum += std.math.powi(u128, digits[i], cursor) catch unreachable;
    }
    return sum == num;
}
