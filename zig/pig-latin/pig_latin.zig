const mem = @import("std").mem;

pub fn translate(allocator: mem.Allocator, phrase: []const u8) mem.Allocator.Error![]u8 {
    const words = try allocator.alloc(u8, phrase.len + (mem.count(u8, phrase, " ") + 1) * 2);
    var i: usize = 0;
    var tokens = mem.tokenizeScalar(u8, phrase, ' ');
    while (tokens.next()) |word| : (i += word.len + 3) {
        var begin = mem.indexOfAny(u8, word, "aeiou") orelse mem.indexOfScalar(u8, word, 'y') orelse 0;
        if (mem.startsWith(u8, word, "yt") or mem.startsWith(u8, word, "xr")) {
            begin = 0;
        } else if (begin > 0 and mem.eql(u8, word[begin - 1 .. begin + 1], "qu")) {
            begin += 1;
        }
        @memcpy(words[i .. i + word.len - begin], word[begin..word.len]);
        @memcpy(words[i + word.len - begin .. i + word.len], word[0..begin]);
        @memcpy(words[i + word.len .. i + word.len + 2], "ay");
        if (i + word.len + 2 < words.len) words[i + word.len + 2] = ' ';
    }
    return words;
}
