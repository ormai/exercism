const std = @import("std");

pub fn rotate(allocator: std.mem.Allocator, text: []const u8, shiftKey: u5) std.mem.Allocator.Error![]u8 {
    const cypher = try allocator.dupe(u8, text);
    for (cypher) |*c| {
        if (std.ascii.isAlphabetic(c.*)) {
            const first: u8 = if (std.ascii.isUpper(c.*)) 'A' else 'a';
            c.* = (c.* - first + shiftKey) % 26 + first;
        }
    }
    return cypher;
}
