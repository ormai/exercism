const std = @import("std");

pub fn isValid(comptime s: []const u8) bool {
    var num: [s.len]u8 = undefined;
    var cursor: usize = 0;
    for (s) |c| {
        num[cursor] = std.fmt.charToDigit(c, 10) catch if (c == ' ') continue else return false;
        cursor += 1;
    }

    var j: usize = 1;
    while (j < cursor) : (j += 2) {
        const double = num[cursor - j - 1] * 2;
        num[cursor - j - 1] = if (double > 9) double - 9 else double;
    }

    var sum: u64 = 0;
    for (num) |n| {
        sum += n;
    }
    return cursor > 1 and sum % 10 == 0;
}
