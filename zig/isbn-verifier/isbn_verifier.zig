const std = @import("std");

pub fn isValidIsbn10(s: []const u8) bool {
    var isbn: [10]u8 = undefined;
    var i: usize = 0;
    for (s) |c| {
        if (i >= 10) {
            return false;
        } else if (c == 'X' and i == 9) {
            isbn[i] = 10;
            i += 1;
        } else if (std.ascii.isDigit(c)) {
            isbn[i] = c - '0';
            i += 1;
        } else if (c != '-') {
            return false;
        }
    }
    if (i != 10) {
        return false;
    }

    var prod: usize = 0;
    for (isbn, 0..) |d, p| {
        prod += d * (10 - p);
    }
    return prod % 11 == 0;
}
