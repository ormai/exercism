const std = @import("std");

pub fn clean(phrase: []const u8) ?[10]u8 {
    var number: [10]u8 = undefined;
    var i: usize = 0;
    for (std.mem.trim(u8, phrase, " ")) |c| {
        if (i >= 10) {
            return null;
        } else if (i == 0 or i == 3 or i == 6) {
            if (c >= '2' and c <= '9') {
                number[i] = c;
                i += 1;
            }
        } else {
            if (c >= '0' and c <= '9') {
                number[i] = c;
                i += 1;
            }
        }
    }
    return if (i < 10) null else number;
}
