const std = @import("std");
const mem = std.mem;

pub fn sum(allocator: mem.Allocator, factors: []const u32, limit: u32) !u64 {
    var set = std.AutoHashMap(u32, void).init(allocator);
    defer set.deinit();

    for (factors) |factor| {
        if (factor > 0) {
            var i = factor;
            while (i < limit) : (i += factor) {
                try set.put(i, {});
            }
        }
    }

    var total: u64 = 0;
    var it = set.keyIterator();
    while (it.next()) |multiple| {
        total += multiple.*;
    }
    return total;
}
