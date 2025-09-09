const std = @import("std");

pub const Coordinate = struct {
    x: f32,
    y: f32,

    pub fn init(x: f32, y: f32) Coordinate {
        return Coordinate{ .x = x, .y = y };
    }

    pub fn score(self: Coordinate) usize {
        const d = std.math.hypot(self.x, self.y);
        return if (d <= 1) 10 else if (d <= 5) 5 else if (d <= 10) 1 else 0;
    }
};
