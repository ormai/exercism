const std = @import("std");

pub fn convert(buffer: []u8, n: u32) []const u8 {
    var end: usize = 0;

    if (n % 3 == 0) append(buffer, "Pling", &end);
    if (n % 5 == 0) append(buffer, "Plang", &end);
    if (n % 7 == 0) append(buffer, "Plong", &end);

    if (end == 0) {
        return std.fmt.bufPrint(buffer, "{d}", .{n}) catch unreachable;
    }
    return buffer[0..end];
}

// Appends s to target and updates end
fn append(target: []u8, s: []const u8, end: *usize) void {
    @memcpy(target[end.* .. end.* + s.len], s);
    end.* += s.len;
}
