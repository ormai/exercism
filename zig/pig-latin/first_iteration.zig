const std = @import("std");
const Allocator = std.mem.Allocator;
const indexOf = std.mem.indexOf;
const tokenizeAny = std.mem.tokenizeAny;
const startsWith = std.mem.startsWith;

pub fn translate(allocator: Allocator, phrase: []const u8) Allocator.Error![]u8 {
    var words = try allocator.alloc(u8, 0);
    var it = tokenizeAny(u8, phrase, " ");
    while (it.next()) |token| {
        var word = try allocator.dupe(u8, token);
        defer allocator.free(word);
        _ = try rule1(allocator, &word) or try rule3(allocator, &word) or try rule4(allocator, &word) or try rule2(allocator, &word);
        if (words.len != 0) try append(allocator, &words, " ");
        try append(allocator, &words, word);
    }
    return words;
}

fn rule4(allocator: Allocator, w: *[]u8) Allocator.Error!bool {
    const y = indexOf(u8, w.*, "y");
    if (y != null and y.? > 0) {
        for (0..y.?) |i| if (isVowel(w.*[i])) return false;
        try shiftBack(allocator, w, 0, y.?);
        try append(allocator, w, "ay");
        return true;
    }
    return false;
}

fn rule3(allocator: Allocator, w: *[]u8) Allocator.Error!bool {
    const qu = indexOf(u8, w.*, "qu");
    if (qu == null) return false;
    for (0..qu.?) |i| if (isVowel(w.*[i])) return false;
    try shiftBack(allocator, w, 0, qu.? + 2);
    try append(allocator, w, "ay");
    return true;
}

fn rule2(allocator: Allocator, word: *[]u8) Allocator.Error!bool {
    var i: usize = 0;
    while (!isVowel(word.*[0]) and i <= word.*.len + 1) : (i += 1) try shiftBack(allocator, word, 0, 1);
    if (i > 0) {
        try append(allocator, word, "ay");
        return true;
    }
    return false;
}

fn rule1(allocator: Allocator, word: *[]u8) Allocator.Error!bool {
    if (isVowel(word.*[0]) or startsWith(u8, word.*, "xr") or startsWith(u8, word.*, "yt")) {
        try append(allocator, word, "ay");
        return true;
    }
    return false;
}

/// Moves word.*[start..end] to the back of the string.
fn shiftBack(allocator: Allocator, word: *[]u8, start: usize, end: usize) Allocator.Error!void {
    // 1. Make a copy of the slice
    const prefix = try allocator.dupe(u8, word.*[start..end]);
    defer allocator.free(prefix);

    // 2. Shift forward what follows the slice to take its place
    for (start..word.*.len - prefix.len) |i| word.*[i] = word.*[i + prefix.len];

    // 3. Put the copy at the end of word
    @memcpy(word.*[word.len - prefix.len .. word.len], prefix);
}

/// Appends suffix to word, reallocates word
fn append(allocator: Allocator, word: *[]u8, suffix: []const u8) Allocator.Error!void {
    word.* = try allocator.realloc(word.*, word.*.len + suffix.len);
    @memcpy(word.*[word.*.len - suffix.len .. word.*.len], suffix);
}

/// Returns true if c is a vowel, false otherwise
fn isVowel(c: u8) bool {
    return switch (c) {
        'a', 'e', 'i', 'o', 'u' => true,
        else => false,
    };
}
