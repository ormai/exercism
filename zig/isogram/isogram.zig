const std = @import("std");

pub fn isIsogram(comptime str: []const u8) bool {
    var set: u26 = 0;
    var word: [str.len]u8 = undefined;
    for (std.ascii.lowerString(&word, str)) |c| {
        if (std.ascii.isAlphabetic(c)) {
            const bit = @as(u26, 1) << @intCast(c - 'a');
            if (set & bit == bit) {
                return false;
            }
            set |= bit;
        }
    }
    return true;
}
