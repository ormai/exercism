const std = @import("std");

pub fn isPangram(str: []const u8) bool {
    const sentence = std.ascii.allocLowerString(std.heap.page_allocator, str) catch unreachable;

    for ("abcdefghijklmnopqrstuvwxyz") |c| {
        if (!std.mem.containsAtLeast(u8, sentence, 1, &[_]u8{c})) {
            return false;
        }
    }
    return true;
}
