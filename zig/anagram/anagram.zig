const std = @import("std");

/// Returns the set of strings in `candidates` that are anagrams of `word`.
/// Caller owns the returned memory.
pub fn detectAnagrams(
    allocator: std.mem.Allocator,
    word: []const u8,
    candidates: []const []const u8,
) !std.BufSet {
    var set = std.BufSet.init(allocator);
    errdefer set.deinit();

    const word_freqs = characterFrequencies(word);
    for (candidates) |candidate| {
        if (!std.ascii.eqlIgnoreCase(word, candidate) and
            std.mem.eql(u32, &characterFrequencies(candidate), &word_freqs))
        {
            try set.insert(candidate);
        }
    }

    return set;
}

fn characterFrequencies(s: []const u8) [26]u32 {
    var freqs = [_]u32{0} ** 26;
    for (s) |c| {
        freqs[std.ascii.toLower(c) - 'a'] += 1;
    }
    return freqs;
}
